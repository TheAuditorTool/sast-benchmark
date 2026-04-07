#!/usr/bin/env python3
"""Chain Detection Benchmark Fidelity Validator v2.0

Validates the chain benchmark's structural integrity using the same
L1-L5 fidelity system as the adversarial benchmark validator.

  L1 -- Structural integrity (CSV <-> annotation cross-reference)
  L2 -- Roundtrip fidelity (scenario directories exist, files on disk)
  L3 -- Schema validation (required fields, valid CWEs, valid categories)
  L4 -- Semantic fidelity (target-line correctness, anti-target-leakage)
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
CSV_FILE = CHAINS_DIR / "expectedresults-0.2.1.csv"
BENCHMARK_PY = SCRIPT_DIR / "convert_theauditor.py"
SCENARIOS_DIR = CHAINS_DIR / "scenarios"

PAT_START = re.compile(r"vuln-code-snippet\s+start\s+(\S+)")
PAT_END = re.compile(r"vuln-code-snippet\s+end\s+(\S+)")
PAT_TARGET_LINE = re.compile(r"vuln-code-snippet\s+target-line\s+(\S+)")

SCAN_EXTENSIONS = {".js", ".py", ".go", ".html", ".json", ".yaml", ".yml"}

VALID_CWES = {
    22, 78, 79, 89, 94, 113, 200, 269, 276, 287, 306, 327, 352, 362,
    384, 434, 502, 601, 611, 732, 798, 862, 863, 915, 918, 942,
}

VALID_CATEGORIES = {
    "unauth_injection",
    "ssrf_pivot",
    "compound_injection",
    "multi_stage",
    "privesc_chain",
    "idor_data_leak",
    "race_condition_bypass",
    "path_traversal_to_read",
    "open_redirect_to_phish",
    "xxe_to_file_read",
    "csrf_to_state_change",
    "header_injection_to_cache_poison",
    "weak_crypto_to_forge",
    "mass_assign_to_privesc",
    "info_leak_to_account_takeover",
    "template_injection_to_rce",
    "hardcoded_creds_to_access",
    "insecure_file_perms_to_tamper",
    "cors_miscfg_to_data_theft",
    "session_fixation_to_hijack",
}

LEAKY_PATTERNS = [
    re.compile(r"CWE-\d+"),
    re.compile(r"VULNERABLE|SAFE variant|IDENTICAL between"),
    re.compile(r"#\s*(BUG|FIXED|VULN|TOCTOU|VULNERABLE|SAFE)\s*:"),
    re.compile(r"--\s*injectable"),
]

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
    target_lines = defaultdict(list)

    if not SCENARIOS_DIR.is_dir():
        errors.append(f"L1 Scenarios directory not found: {SCENARIOS_DIR}")
        return annotations, target_lines

    for root, dirs, files in os.walk(SCENARIOS_DIR):
        dirs[:] = [d for d in dirs if d not in (".git", "__pycache__", ".pf", "node_modules", "migration")]
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

                m = PAT_TARGET_LINE.search(line)
                if m:
                    target_lines[m.group(1)].append((rel, i))

    for key, (file, line) in open_snippets.items():
        errors.append(f"L1 Unclosed annotation start for '{key}' in {file}:{line}")

    return annotations, target_lines


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

    # Check scenario directory structure from annotation file paths
    for key, info in csv_entries.items():
        if key not in annotations:
            continue

        ann_file = annotations[key]["file"]
        path_parts = ann_file.split("/")
        # Expected: scenarios/<scenario_dir>/<variant_dir>/...
        if len(path_parts) >= 4 and path_parts[0] == "scenarios":
            scenario_dir = CHAINS_DIR / "scenarios" / path_parts[1]
            variant_dir = scenario_dir / path_parts[2]
            if not scenario_dir.is_dir():
                errors.append(f"L2 Scenario directory not found: {scenario_dir}")
            elif not variant_dir.is_dir():
                errors.append(f"L2 Variant directory not found: {variant_dir}")


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
def check_semantics(csv_entries, annotations, target_lines):
    """Verify target-line markers exist and check anti-target-leakage."""
    for key, info in csv_entries.items():
        if key not in annotations:
            continue

        ann = annotations[key]
        has_target = key in target_lines

        if not has_target:
            errors.append(
                f"L4 Key '{key}' has NO target-line marker "
                f"in {ann['file']}:{ann['start']}-{ann['end']}"
            )

        for (mfile, mline) in target_lines.get(key, []):
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

    # L4b: Anti-target-leakage check
    leakage_hits = 0
    for root, dirs, files in os.walk(SCENARIOS_DIR):
        dirs[:] = [d for d in dirs if d not in (".git", "__pycache__", "migration")]
        for fn in files:
            if not fn.endswith(".py"):
                continue
            filepath = Path(root) / fn
            try:
                content = filepath.read_text(encoding="utf-8", errors="replace")
            except OSError:
                continue

            rel = str(filepath.relative_to(CHAINS_DIR))
            for pat in LEAKY_PATTERNS:
                if pat.search(content):
                    errors.append(f"L4 Target leakage in {rel}: matches /{pat.pattern}/")
                    leakage_hits += 1
                    break

    if leakage_hits == 0:
        pass  # clean


# ============================================================================
# L5: Scoring Pipeline Readiness
# ============================================================================
def check_scoring_pipeline(all_categories):
    """Verify all chain categories are mapped in convert_theauditor.py."""
    if not BENCHMARK_PY.exists():
        warnings.append("L5 convert_theauditor.py not found - cannot verify scoring pipeline")
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
                f"in convert_theauditor.py CHAIN_RULE_MAP or CHAIN_SINK_MAP"
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
    print("  Chain Detection Benchmark Fidelity Validator v2.0")
    print("  L1-L5 fidelity system")
    print("=" * 64)
    print()

    # --- L1 ---
    print("[L1] Structural Integrity (CSV <-> annotation cross-reference)")
    csv_entries = parse_csv()
    annotations, target_lines = scan_annotations()

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
    print("[L4] Semantic Fidelity (target-line correctness, anti-target-leakage)")
    check_semantics(csv_entries, annotations, target_lines)
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
