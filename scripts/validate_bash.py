#!/usr/bin/env python3
"""Bash SAST Benchmark Fidelity Validator v2.0

Modeled after TheAuditor's multi-level fidelity system:
  L1 — Structural integrity (YAML <-> annotation cross-reference)
  L2 — Roundtrip fidelity (file paths, line markers, snippet content)
  L3 — Schema validation (required fields, valid CWEs, valid categories)
  L4 — Semantic fidelity (vuln-line vs safe-line correctness, overlap detection)
  L5 — Scoring pipeline readiness (RULE_MAP/SINK_MAP coverage)

Exit 0 if all checks pass, 1 if any ERRORS, 2 if only WARNINGS.
No dependencies — stdlib only.
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
BASH_DIR = BENCH_ROOT / "bash"
YAML_FILE = BASH_DIR / "bash_ground_truth.yml"
BENCHMARK_PY = BASH_DIR / "bash_benchmark.py"
SCAN_DIRS = [BASH_DIR / "apps", BASH_DIR / "testcode"]

PAT_START = re.compile(r"vuln-code-snippet\s+start\s+(\S+)")
PAT_END = re.compile(r"vuln-code-snippet\s+end\s+(\S+)")
PAT_VULN_LINE = re.compile(r"vuln-code-snippet\s+vuln-line\s+(\S+)")
PAT_SAFE_LINE = re.compile(r"vuln-code-snippet\s+safe-line\s+(\S+)")

REQUIRED_FIELDS = {"key", "file", "category", "cwe", "vulnerable"}
VALID_CWES = {
    20, 22, 77, 78, 79, 88, 89, 90, 93, 94, 117, 119, 190, 200, 269, 276,
    287, 295, 306, 327, 328, 330, 352, 362, 367, 377, 434, 494, 501, 502,
    532, 601, 611, 614, 643, 732, 798, 862, 863, 918, 943, 1333,
}

errors = []
warnings = []


# ============================================================================
# L1: Structural Integrity
# ============================================================================
def parse_yaml():
    """Parse bash_ground_truth.yml. Returns dict of {key: {fields...}}."""
    entries = {}
    current = None
    seen_keys = {}

    with open(YAML_FILE, "r", encoding="utf-8") as f:
        for line_num, line in enumerate(f, 1):
            stripped = line.strip()
            if not stripped or stripped.startswith("#"):
                continue
            if stripped == "test_cases:":
                continue

            if stripped.startswith("- key:"):
                if current:
                    entries[current["key"]] = current
                key = stripped.split(":", 1)[1].strip()
                if key in seen_keys:
                    errors.append(
                        f"L1 YAML duplicate key '{key}' at line {line_num} "
                        f"(first at line {seen_keys[key]})"
                    )
                seen_keys[key] = line_num
                current = {"key": key, "_yaml_line": line_num}
                continue

            if current and ":" in stripped and not stripped.startswith("-"):
                k, _, v = stripped.partition(":")
                k = k.strip()
                v = v.strip().strip('"').strip("'")
                if k == "vulnerable":
                    current[k] = v.lower() == "true"
                elif k == "cwe":
                    try:
                        current[k] = int(v)
                    except ValueError:
                        errors.append(
                            f"L3 Invalid CWE value '{v}' for key '{current['key']}' "
                            f"at YAML line {line_num}"
                        )
                        current[k] = -1
                elif k in ("file", "category", "description"):
                    current[k] = v

    if current:
        entries[current["key"]] = current

    return entries


def scan_annotations():
    """Scan .sh files for vuln-code-snippet markers.
    Returns:
      annotations: {key: {file, start, end}}
      vuln_lines: {key: [(file, line_num), ...]}
      safe_lines: {key: [(file, line_num), ...]}
      file_lines: {rel_path: [line1, line2, ...]}  (cached for L2 checks)
    """
    annotations = {}
    open_snippets = {}
    vuln_lines = defaultdict(list)
    safe_lines = defaultdict(list)
    file_lines = {}

    for scan_dir in SCAN_DIRS:
        if not scan_dir.is_dir():
            continue
        for root, dirs, files in os.walk(scan_dir):
            dirs[:] = [d for d in dirs if d not in (".git", "node_modules", ".auditor_venv", ".pf")]
            for fn in sorted(files):
                if not fn.endswith(".sh"):
                    continue
                filepath = Path(root) / fn
                rel = str(filepath.relative_to(BASH_DIR)).replace("\\", "/")
                try:
                    with open(filepath, "r", encoding="utf-8", errors="replace") as f:
                        lines = f.readlines()
                except OSError:
                    continue

                file_lines[rel] = lines

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

    return annotations, vuln_lines, safe_lines, file_lines


# ============================================================================
# L2: Roundtrip Fidelity
# ============================================================================
def check_roundtrip(yaml_entries, annotations, file_lines):
    """Verify YAML file paths match annotation file paths, and files exist."""
    for key, info in yaml_entries.items():
        if key not in annotations:
            continue  # already caught by L1 orphan check

        yaml_file = info.get("file", "")
        ann_file = annotations[key]["file"]

        # File path mismatch
        if yaml_file != ann_file:
            errors.append(
                f"L2 File mismatch for '{key}': YAML says '{yaml_file}', "
                f"annotation is in '{ann_file}'"
            )

        # File exists on disk
        full_path = BASH_DIR / yaml_file
        if not full_path.exists():
            errors.append(f"L2 File not found on disk: '{yaml_file}' (key '{key}')")

        # Empty snippet (start and end on adjacent lines, no code between)
        ann = annotations[key]
        if ann["end"] - ann["start"] <= 1:
            warnings.append(
                f"L2 Empty snippet for '{key}' in {ann_file}:{ann['start']}-{ann['end']} "
                f"(no code between start/end markers)"
            )


# ============================================================================
# L3: Schema Validation
# ============================================================================
def check_schema(yaml_entries):
    """Validate required fields, CWE values, and category consistency."""
    all_categories = set()

    for key, info in yaml_entries.items():
        # Required fields
        missing = REQUIRED_FIELDS - set(info.keys())
        if missing:
            errors.append(
                f"L3 Missing required fields for '{key}': {sorted(missing)} "
                f"(YAML line {info.get('_yaml_line', '?')})"
            )

        # CWE validity
        cwe = info.get("cwe")
        if cwe is not None and cwe not in VALID_CWES:
            warnings.append(
                f"L3 CWE {cwe} for key '{key}' is not in the known CWE set "
                f"(may be valid but not in our allowlist)"
            )

        # Category tracking
        cat = info.get("category")
        if cat:
            all_categories.add(cat)

        # Description should not be empty
        desc = info.get("description", "")
        if not desc:
            warnings.append(f"L3 Empty description for key '{key}'")

    return all_categories


# ============================================================================
# L4: Semantic Fidelity
# ============================================================================
def check_semantics(yaml_entries, annotations, vuln_lines, safe_lines):
    """Verify vuln-line/safe-line markers match vulnerable classification."""
    for key, info in yaml_entries.items():
        if key not in annotations:
            continue

        is_vuln = info.get("vulnerable", None)
        ann = annotations[key]
        has_vuln_marker = key in vuln_lines
        has_safe_marker = key in safe_lines

        # TP must have vuln-line, TN must have safe-line
        if is_vuln is True:
            if not has_vuln_marker:
                errors.append(
                    f"L4 Key '{key}' is vulnerable=true but has NO vuln-line marker "
                    f"in {ann['file']}:{ann['start']}-{ann['end']}"
                )
            if has_safe_marker:
                errors.append(
                    f"L4 Key '{key}' is vulnerable=true but has a safe-line marker "
                    f"(should be vuln-line)"
                )
        elif is_vuln is False:
            if not has_safe_marker:
                errors.append(
                    f"L4 Key '{key}' is vulnerable=false but has NO safe-line marker "
                    f"in {ann['file']}:{ann['start']}-{ann['end']}"
                )
            if has_vuln_marker:
                errors.append(
                    f"L4 Key '{key}' is vulnerable=false but has a vuln-line marker "
                    f"(should be safe-line)"
                )

        # Vuln-line/safe-line must be INSIDE the snippet range
        for marker_key, marker_list in [(key, vuln_lines.get(key, [])), (key, safe_lines.get(key, []))]:
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

    # Overlapping snippet ranges within the same file
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
    """Verify all benchmark categories are in bash_benchmark.py's RULE_MAP or SINK_MAP."""
    if not BENCHMARK_PY.exists():
        warnings.append("L5 bash_benchmark.py not found - cannot verify scoring pipeline")
        return

    with open(BENCHMARK_PY, "r", encoding="utf-8") as f:
        scoring_content = f.read()

    # Extract categories from RULE_MAP and SINK_MAP values
    mapped_categories = set()
    for m in re.finditer(r':\s*"([a-z_]+)"', scoring_content):
        mapped_categories.add(m.group(1))

    for cat in sorted(all_categories):
        if cat not in mapped_categories:
            warnings.append(
                f"L5 Category '{cat}' exists in ground truth but is not mapped "
                f"in bash_benchmark.py RULE_MAP or SINK_MAP"
            )


# ============================================================================
# Report
# ============================================================================
def print_report(yaml_entries, annotations):
    """Print the full fidelity report."""

    categories = defaultdict(lambda: {"tp": 0, "tn": 0})
    for key, info in yaml_entries.items():
        cat = info.get("category", "unknown")
        if info.get("vulnerable", False):
            categories[cat]["tp"] += 1
        else:
            categories[cat]["tn"] += 1

    total_tp = sum(c["tp"] for c in categories.values())
    total_tn = sum(c["tn"] for c in categories.values())
    total = total_tp + total_tn

    print(f"YAML entries:   {len(yaml_entries)}")
    print(f"Annotations:    {len(annotations)}")
    print(f"Match:          {'YES' if len(yaml_entries) == len(annotations) else 'NO - MISMATCH'}")
    print(f"Total TP:       {total_tp}")
    print(f"Total TN:       {total_tn}")
    if total > 0:
        print(f"TP/TN split:    {total_tp * 100.0 / total:.1f}% / {total_tn * 100.0 / total:.1f}%")
    print()

    print(f"{'Category':<20} {'CWE':>5} {'TP':>4} {'TN':>4} {'Total':>6} {'Balance':>8}")
    print("-" * 52)

    # Collect CWE per category
    cat_cwes = {}
    for key, info in yaml_entries.items():
        cat = info.get("category")
        cwe = info.get("cwe")
        if cat and cwe:
            cat_cwes[cat] = cwe

    for cat in sorted(categories.keys()):
        tp = categories[cat]["tp"]
        tn = categories[cat]["tn"]
        cat_total = tp + tn
        cwe = cat_cwes.get(cat, "?")
        if cat_total > 0:
            balance = f"{tp * 100 // cat_total}/{tn * 100 // cat_total}"
        else:
            balance = "N/A"
        print(f"{cat:<20} {cwe:>5} {tp:>4} {tn:>4} {cat_total:>6} {balance:>8}")

    print("-" * 52)
    balance_str = f"{total_tp * 100 // total}/{total_tn * 100 // total}" if total > 0 else "N/A"
    print(f"{'TOTAL':<20} {'':>5} {total_tp:>4} {total_tn:>4} {total:>6} {balance_str:>8}")
    print()


def main():
    print("=" * 64)
    print("  Bash SAST Benchmark Fidelity Validator v2.0")
    print("  Modeled after TheAuditor fidelity system (L1-L5)")
    print("=" * 64)
    print()

    # --- L1: Structural Integrity ---
    print("[L1] Structural Integrity (YAML <-> annotation cross-reference)")
    yaml_entries = parse_yaml()
    annotations, vuln_lines, safe_lines, file_lines = scan_annotations()

    yaml_keys = set(yaml_entries.keys())
    ann_keys = set(annotations.keys())

    for key in sorted(yaml_keys - ann_keys):
        errors.append(f"L1 Orphan YAML: '{key}' in ground truth but no annotation in source")
    for key in sorted(ann_keys - yaml_keys):
        errors.append(f"L1 Orphan annotation: '{key}' in source but no YAML entry")

    l1_errors = len(errors)
    print(f"  Checks: duplicate keys, orphans, balanced start/end, unclosed snippets")
    print(f"  Result: {l1_errors} errors found")
    print()

    # --- L2: Roundtrip Fidelity ---
    print("[L2] Roundtrip Fidelity (file path match, file existence, empty snippets)")
    check_roundtrip(yaml_entries, annotations, file_lines)
    l2_errors = len(errors) - l1_errors
    print(f"  Checks: YAML file field matches annotation file, files exist on disk")
    print(f"  Result: {l2_errors} errors found")
    print()

    # --- L3: Schema Validation ---
    print("[L3] Schema Validation (required fields, valid CWEs, descriptions)")
    all_categories = check_schema(yaml_entries)
    l3_errors = len(errors) - l1_errors - l2_errors
    print(f"  Checks: required fields present, CWE in valid set, non-empty descriptions")
    print(f"  Categories found: {len(all_categories)} ({', '.join(sorted(all_categories))})")
    print(f"  Result: {l3_errors} errors found")
    print()

    # --- L4: Semantic Fidelity ---
    print("[L4] Semantic Fidelity (vuln-line/safe-line correctness, overlap detection)")
    check_semantics(yaml_entries, annotations, vuln_lines, safe_lines)
    l4_errors = len(errors) - l1_errors - l2_errors - l3_errors
    print(f"  Checks: TP has vuln-line, TN has safe-line, markers inside snippet, no overlaps")
    print(f"  Result: {l4_errors} errors found")
    print()

    # --- L5: Scoring Pipeline Readiness ---
    print("[L5] Scoring Pipeline Readiness (RULE_MAP/SINK_MAP coverage)")
    check_scoring_pipeline(all_categories)
    l5_warnings = len(warnings)
    print(f"  Checks: every category has a mapping in bash_benchmark.py")
    print(f"  Result: {l5_warnings} warnings")
    print()

    # --- Summary ---
    print("=" * 64)
    print("  SUMMARY")
    print("=" * 64)
    print()

    print_report(yaml_entries, annotations)

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
        print(f"  {len(errors)} errors must be fixed before benchmark is valid.")
        return 1
    elif warnings:
        print("RESULT: PASS WITH WARNINGS")
        print(f"  {len(warnings)} warnings found. Review recommended but not blocking.")
        return 2
    else:
        print("RESULT: PASS")
        print("  All L1-L5 fidelity checks passed. Benchmark is valid.")
        return 0


if __name__ == "__main__":
    sys.exit(main())
