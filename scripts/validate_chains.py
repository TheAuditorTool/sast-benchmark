#!/usr/bin/env python3
"""Chain Detection Benchmark Fidelity Validator v1.0

Validates the chain benchmark's structural integrity using the same
L1-L5 fidelity system as the adversarial benchmark validator.

  L1 -- Structural integrity (CSV <-> annotation cross-reference)
  L2 -- Roundtrip fidelity (scenario directories exist, files on disk)
  L3 -- Schema validation (required fields, valid CWEs, valid categories)
  L4 -- Semantic fidelity (vuln-line/safe-line correctness, overlap detection)
  L5 -- Scoring pipeline readiness (CHAIN_RULE_MAP coverage)

Exit 0 if all checks pass, 1 if any ERRORS, 2 if only WARNINGS.
No dependencies -- stdlib only.
"""

import os
import re
import sys
from collections import defaultdict
from pathlib import Path

# ============================================================================
# Configuration
# ============================================================================
SCRIPT_DIR = Path(__file__).resolve().parent
BENCH_ROOT = SCRIPT_DIR.parent
CHAINS_DIR = BENCH_ROOT / "chains"
CSV_FILE = CHAINS_DIR / "expectedresults-0.1.0.csv"
BENCHMARK_PY = SCRIPT_DIR / "convert_theauditor.py"
SCENARIOS_DIR = CHAINS_DIR / "scenarios"

PAT_START = re.compile(r"vuln-code-snippet\s+start\s+(\S+)")
PAT_END = re.compile(r"vuln-code-snippet\s+end\s+(\S+)")
PAT_VULN_LINE = re.compile(r"vuln-code-snippet\s+vuln-line\s+(\S+)")
PAT_SAFE_LINE = re.compile(r"vuln-code-snippet\s+safe-line\s+(\S+)")

SCAN_EXTENSIONS = {".js", ".py", ".go", ".html", ".json", ".yaml", ".yml"}

VALID_CWES = {
    22, 78, 79, 89, 94, 200, 269, 276, 287, 306, 327, 352, 362,
    434, 502, 601, 611, 732, 798, 862, 863, 918,
}

VALID_CATEGORIES = {
    "unauth_injection",
    "ssrf_pivot",
    "compound_injection",
    "multi_stage",
}

errors = []
warnings = []


# ============================================================================
# L1: Structural Integrity
# ============================================================================
def parse_csv():
    """Parse expectedresults CSV."""
    entries = {}
    seen_keys = {}

    with open(CSV_FILE, "r", encoding="utf-8") as f:
        for line_num, line in enumerate(f, 1):
            stripped = line.strip()
            if not stripped or stripped.startswith("#"):
                continue

            parts = stripped.split(",")
            if len(parts) < 4:
                errors.append(f"L1 CSV line {line_num}: expected 4 columns, got {len(parts)}")
                continue

            key = parts[0].strip()
            category = parts[1].strip()
            vulnerable_str = parts[2].strip().lower()
            cwe_str = parts[3].strip()

            if key in seen_keys:
                errors.append(
                    f"L1 CSV duplicate key '{key}' at line {line_num} "
                    f"(first at line {seen_keys[key]})"
                )
            seen_keys[key] = line_num

            try:
                cwe = int(cwe_str)
            except ValueError:
                errors.append(f"L3 Invalid CWE value '{cwe_str}' for key '{key}' at CSV line {line_num}")
                cwe = -1

            entries[key] = {
                "key": key,
                "category": category,
                "vulnerable": vulnerable_str == "true",
                "cwe": cwe,
                "_csv_line": line_num,
            }

    return entries


def scan_annotations():
    """Scan scenario source files for vuln-code-snippet markers."""
    annotations = {}
    open_snippets = {}
    vuln_lines = defaultdict(list)
    safe_lines = defaultdict(list)

    if not SCENARIOS_DIR.is_dir():
        errors.append(f"L1 Scenarios directory not found: {SCENARIOS_DIR}")
        return annotations, vuln_lines, safe_lines

    for root, dirs, files in os.walk(SCENARIOS_DIR):
        dirs[:] = [d for d in dirs if d not in (".git", "__pycache__", ".pf", "node_modules")]
        for fn in sorted(files):
            ext = os.path.splitext(fn)[1].lower()
            if ext not in SCAN_EXTENSIONS:
                continue
            filepath = Path(root) / fn
            rel = str(filepath.relative_to(CHAINS_DIR)).replace("\\", "/")
            try:
                with open(filepath, "r", encoding="utf-8", errors="replace") as f:
                    lines = f.readlines()
            except OSError:
                continue

            for i, line in enumerate(lines, 1):
                m = PAT_START.search(line)
                if m:
                    key = m.group(1)
                    if key in open_snippets:
                        errors.append(
                            f"L1 Duplicate annotation start for '{key}' in {rel}:{i} "
                            f"(first at {open_snippets[key][0]}:{open_snippets[key][1]})"
                        )
                    open_snippets[key] = (rel, i)

                m = PAT_END.search(line)
                if m:
                    key = m.group(1)
                    if key in open_snippets:
                        start_file, start_line = open_snippets.pop(key)
                        if key in annotations:
                            errors.append(f"L1 Duplicate annotation key '{key}' in {rel}:{i}")
                        annotations[key] = {"file": start_file, "start": start_line, "end": i}
                    else:
                        errors.append(f"L1 End without start for '{key}' in {rel}:{i}")

                m = PAT_VULN_LINE.search(line)
                if m:
                    vuln_lines[m.group(1)].append((rel, i))

                m = PAT_SAFE_LINE.search(line)
                if m:
                    safe_lines[m.group(1)].append((rel, i))

    for key, (file, line) in open_snippets.items():
        errors.append(f"L1 Unclosed annotation start for '{key}' in {file}:{line}")

    return annotations, vuln_lines, safe_lines


# ============================================================================
# L2: Roundtrip Fidelity
# ============================================================================
def check_roundtrip(csv_entries, annotations):
    """Verify scenario directories exist and annotations are non-empty."""
    for key, info in csv_entries.items():
        if key not in annotations:
            continue

        ann = annotations[key]
        full_path = CHAINS_DIR / ann["file"]
        if not full_path.exists():
            errors.append(f"L2 File not found on disk: '{ann['file']}' (key '{key}')")

        if ann["end"] - ann["start"] <= 1:
            warnings.append(
                f"L2 Empty snippet for '{key}' in {ann['file']}:{ann['start']}-{ann['end']}"
            )

    # Check scenario directory structure
    for key, info in csv_entries.items():
        # Derive scenario name and variant from key: chain_<scenario>_<vuln|safe>
        parts = key.rsplit("_", 1)
        if len(parts) != 2 or parts[1] not in ("vuln", "safe"):
            warnings.append(f"L2 Key '{key}' does not follow chain_<name>_<vuln|safe> pattern")
            continue

        # Extract scenario directory name from the annotation file path
        if key in annotations:
            ann_file = annotations[key]["file"]
            # Expected: scenarios/<scenario_name>/<variant>/...
            path_parts = ann_file.split("/")
            if len(path_parts) >= 3 and path_parts[0] == "scenarios":
                scenario_dir = CHAINS_DIR / "scenarios" / path_parts[1] / parts[1]
                if not scenario_dir.is_dir():
                    errors.append(f"L2 Scenario variant directory not found: {scenario_dir}")


# ============================================================================
# L3: Schema Validation
# ============================================================================
def check_schema(csv_entries):
    """Validate required fields, CWE values, and category consistency."""
    all_categories = set()

    for key, info in csv_entries.items():
        cwe = info.get("cwe")
        if cwe is not None and cwe not in VALID_CWES:
            warnings.append(
                f"L3 CWE {cwe} for key '{key}' is not in the known chain CWE set"
            )

        cat = info.get("category")
        if cat:
            all_categories.add(cat)
            if cat not in VALID_CATEGORIES:
                errors.append(
                    f"L3 Unknown category '{cat}' for key '{key}' "
                    f"(valid: {', '.join(sorted(VALID_CATEGORIES))})"
                )

    return all_categories


# ============================================================================
# L4: Semantic Fidelity
# ============================================================================
def check_semantics(csv_entries, annotations, vuln_lines, safe_lines):
    """Verify vuln-line/safe-line markers match chain exploitability."""
    for key, info in csv_entries.items():
        if key not in annotations:
            continue

        is_vuln = info.get("vulnerable", None)
        ann = annotations[key]
        has_vuln_marker = key in vuln_lines
        has_safe_marker = key in safe_lines

        if is_vuln is True:
            if not has_vuln_marker:
                errors.append(
                    f"L4 Key '{key}' is chain_exploitable=true but has NO vuln-line marker "
                    f"in {ann['file']}:{ann['start']}-{ann['end']}"
                )
            if has_safe_marker:
                errors.append(
                    f"L4 Key '{key}' is chain_exploitable=true but has a safe-line marker"
                )
        elif is_vuln is False:
            if not has_safe_marker:
                errors.append(
                    f"L4 Key '{key}' is chain_exploitable=false but has NO safe-line marker "
                    f"in {ann['file']}:{ann['start']}-{ann['end']}"
                )
            if has_vuln_marker:
                errors.append(
                    f"L4 Key '{key}' is chain_exploitable=false but has a vuln-line marker"
                )

        for marker_list in [vuln_lines.get(key, []), safe_lines.get(key, [])]:
            for (mfile, mline) in marker_list:
                if mfile != ann["file"]:
                    errors.append(
                        f"L4 Marker for '{key}' at {mfile}:{mline} is in a different file "
                        f"than the snippet ({ann['file']})"
                    )
                elif not (ann["start"] <= mline <= ann["end"]):
                    errors.append(
                        f"L4 Marker for '{key}' at {mfile}:{mline} is OUTSIDE snippet range "
                        f"{ann['start']}-{ann['end']}"
                    )

    # Overlapping snippets within same file
    by_file = defaultdict(list)
    for key, ann in annotations.items():
        by_file[ann["file"]].append((ann["start"], ann["end"], key))

    for file, ranges in by_file.items():
        ranges.sort()
        for i in range(len(ranges) - 1):
            s1, e1, k1 = ranges[i]
            s2, e2, k2 = ranges[i + 1]
            if s2 <= e1:
                errors.append(
                    f"L4 Overlapping snippets in {file}: '{k1}' ({s1}-{e1}) "
                    f"overlaps with '{k2}' ({s2}-{e2})"
                )


# ============================================================================
# L5: Scoring Pipeline Readiness
# ============================================================================
def check_scoring_pipeline(all_categories):
    """Verify all chain categories are mapped in chain_benchmark.py."""
    if not BENCHMARK_PY.exists():
        warnings.append("L5 chain_benchmark.py not found - cannot verify scoring pipeline")
        return

    with open(BENCHMARK_PY, "r", encoding="utf-8") as f:
        scoring_content = f.read()

    mapped_categories = set()
    for m in re.finditer(r'"([a-z][a-z0-9_]+)"', scoring_content):
        mapped_categories.add(m.group(1))

    for cat in sorted(all_categories):
        if cat not in mapped_categories:
            warnings.append(
                f"L5 Category '{cat}' exists in ground truth but is not mapped "
                f"in chain_benchmark.py CHAIN_RULE_MAP or CHAIN_SINK_MAP"
            )


# ============================================================================
# Report
# ============================================================================
def print_report(csv_entries, annotations):
    """Print the fidelity report."""
    categories = defaultdict(lambda: {"tp": 0, "tn": 0})
    for key, info in csv_entries.items():
        cat = info.get("category", "unknown")
        if info.get("vulnerable", False):
            categories[cat]["tp"] += 1
        else:
            categories[cat]["tn"] += 1

    total_tp = sum(c["tp"] for c in categories.values())
    total_tn = sum(c["tn"] for c in categories.values())
    total = total_tp + total_tn

    print(f"CSV entries:    {len(csv_entries)}")
    print(f"Annotations:    {len(annotations)}")
    print(f"Match:          {'YES' if len(csv_entries) == len(annotations) else 'NO - MISMATCH'}")
    print(f"Total chains:   {total_tp} exploitable / {total_tn} mitigated")
    if total > 0:
        print(f"Balance:        {total_tp * 100.0 / total:.1f}% / {total_tn * 100.0 / total:.1f}%")
    print()

    print(f"{'Category':<24} {'CWE':>5} {'Vuln':>5} {'Safe':>5} {'Total':>6}")
    print("-" * 50)

    cat_cwes = {}
    for key, info in csv_entries.items():
        cat = info.get("category")
        cwe = info.get("cwe")
        if cat and cwe:
            cat_cwes.setdefault(cat, set()).add(cwe)

    for cat in sorted(categories.keys()):
        tp = categories[cat]["tp"]
        tn = categories[cat]["tn"]
        cwes = cat_cwes.get(cat, set())
        cwe_str = ",".join(str(c) for c in sorted(cwes))
        print(f"{cat:<24} {cwe_str:>5} {tp:>5} {tn:>5} {tp + tn:>6}")

    print("-" * 50)
    print(f"{'TOTAL':<24} {'':>5} {total_tp:>5} {total_tn:>5} {total:>6}")
    print()


def main():
    print("=" * 64)
    print("  Chain Detection Benchmark Fidelity Validator v1.0")
    print("  L1-L5 fidelity system")
    print("=" * 64)
    print()

    # --- L1 ---
    print("[L1] Structural Integrity (CSV <-> annotation cross-reference)")
    csv_entries = parse_csv()
    annotations, vuln_lines, safe_lines = scan_annotations()

    csv_keys = set(csv_entries.keys())
    ann_keys = set(annotations.keys())

    for key in sorted(csv_keys - ann_keys):
        errors.append(f"L1 Orphan CSV: '{key}' in ground truth but no annotation in source")
    for key in sorted(ann_keys - csv_keys):
        errors.append(f"L1 Orphan annotation: '{key}' in source but no CSV entry")

    l1_errors = len(errors)
    print(f"  Result: {l1_errors} errors found")
    print()

    # --- L2 ---
    print("[L2] Roundtrip Fidelity (scenario dirs exist, files on disk)")
    check_roundtrip(csv_entries, annotations)
    l2_errors = len(errors) - l1_errors
    print(f"  Result: {l2_errors} errors found")
    print()

    # --- L3 ---
    print("[L3] Schema Validation (CWEs, categories)")
    all_categories = check_schema(csv_entries)
    l3_errors = len(errors) - l1_errors - l2_errors
    print(f"  Categories: {len(all_categories)} ({', '.join(sorted(all_categories))})")
    print(f"  Result: {l3_errors} errors found")
    print()

    # --- L4 ---
    print("[L4] Semantic Fidelity (vuln-line/safe-line correctness)")
    check_semantics(csv_entries, annotations, vuln_lines, safe_lines)
    l4_errors = len(errors) - l1_errors - l2_errors - l3_errors
    print(f"  Result: {l4_errors} errors found")
    print()

    # --- L5 ---
    print("[L5] Scoring Pipeline Readiness")
    check_scoring_pipeline(all_categories)
    l5_warnings = len(warnings)
    print(f"  Result: {l5_warnings} warnings")
    print()

    # --- Summary ---
    print("=" * 64)
    print("  SUMMARY")
    print("=" * 64)
    print()

    print_report(csv_entries, annotations)

    if errors:
        print(f"ERRORS: {len(errors)}")
        for err in errors:
            print(f"  [ERROR] {err}")
        print()

    if warnings:
        print(f"WARNINGS: {len(warnings)}")
        for warn in warnings:
            print(f"  [WARN]  {warn}")
        print()

    if errors:
        print("RESULT: FAIL")
        return 1
    elif warnings:
        print("RESULT: PASS WITH WARNINGS")
        return 2
    else:
        print("RESULT: PASS")
        return 0


if __name__ == "__main__":
    sys.exit(main())
