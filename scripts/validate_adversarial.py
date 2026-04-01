#!/usr/bin/env python3
"""Adversarial Evasion Benchmark Fidelity Validator v1.0

Modeled after TheAuditor's multi-level fidelity system:
  L1 -- Structural integrity (CSV <-> annotation cross-reference)
  L2 -- Roundtrip fidelity (file paths, line markers, snippet content)
  L3 -- Schema validation (required fields, valid CWEs, valid categories)
  L4 -- Semantic fidelity (vuln-line vs safe-line correctness, overlap detection)
  L5 -- Scoring pipeline readiness (SIGNAL_MAP/RULE_MAP coverage)

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
ADV_DIR = BENCH_ROOT / "adversarial"
CSV_FILE = ADV_DIR / "expectedresults-0.2.0.csv"
BENCHMARK_PY = SCRIPT_DIR / "convert_theauditor.py"
SCAN_DIRS = [ADV_DIR / "testcode"]

PAT_START = re.compile(r"vuln-code-snippet\s+start\s+(\S+)")
PAT_END = re.compile(r"vuln-code-snippet\s+end\s+(\S+)")
PAT_VULN_LINE = re.compile(r"vuln-code-snippet\s+vuln-line\s+(\S+)")
PAT_SAFE_LINE = re.compile(r"vuln-code-snippet\s+safe-line\s+(\S+)")

# File extensions to scan (cross-language benchmark)
SCAN_EXTENSIONS = {".js", ".py", ".go", ".rs", ".php", ".rb", ".sh", ".json", ".txt", ".md", ".toml", ".yaml", ".yml"}

REQUIRED_FIELDS = {"key", "category", "cwe", "vulnerable"}
VALID_CWES = {
    20, 22, 77, 78, 79, 88, 89, 90, 93, 94, 116, 117, 119, 176, 190, 200,
    269, 276, 287, 295, 306, 327, 328, 330, 352, 362, 367, 377, 434, 451,
    494, 501, 502, 506, 532, 601, 611, 614, 643, 732, 798, 829, 838, 862,
    863, 918, 943, 1059, 1333,
}

VALID_CATEGORIES = {
    "unicode_payload",
    "visual_deception",
    "dynamic_construction",
    "supply_chain",
    "ai_prompt_injection",
    "c2_fingerprint",
    "charset_mapping",
    "steganographic_payload",
    "slopsquatting",
    "llm_code_generation",
}

errors = []
warnings = []


# ============================================================================
# L1: Structural Integrity
# ============================================================================
def parse_csv():
    """Parse expectedresults CSV. Format: test name,category,real vulnerability,CWE"""
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
    """Scan source files for vuln-code-snippet markers."""
    annotations = {}
    open_snippets = {}
    vuln_lines = defaultdict(list)
    safe_lines = defaultdict(list)
    file_lines = {}

    for scan_dir in SCAN_DIRS:
        if not scan_dir.is_dir():
            continue
        for root, dirs, files in os.walk(scan_dir):
            dirs[:] = [d for d in dirs if d not in (".git", "node_modules", ".auditor_venv", ".pf", "__pycache__")]
            for fn in sorted(files):
                ext = os.path.splitext(fn)[1].lower()
                if ext not in SCAN_EXTENSIONS:
                    continue
                filepath = Path(root) / fn
                rel = str(filepath.relative_to(ADV_DIR)).replace("\\", "/")
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
def check_roundtrip(csv_entries, annotations, file_lines):
    """Verify annotation files exist on disk and snippets are non-empty."""
    for key, info in csv_entries.items():
        if key not in annotations:
            continue

        ann = annotations[key]
        ann_file = ann["file"]

        full_path = ADV_DIR / ann_file
        if not full_path.exists():
            errors.append(f"L2 File not found on disk: '{ann_file}' (key '{key}')")

        if ann["end"] - ann["start"] <= 1:
            warnings.append(
                f"L2 Empty snippet for '{key}' in {ann_file}:{ann['start']}-{ann['end']} "
                f"(no code between start/end markers)"
            )


# ============================================================================
# L3: Schema Validation
# ============================================================================
def check_schema(csv_entries):
    """Validate required fields, CWE values, and category consistency."""
    all_categories = set()

    for key, info in csv_entries.items():
        missing = REQUIRED_FIELDS - set(info.keys())
        if missing:
            errors.append(
                f"L3 Missing required fields for '{key}': {sorted(missing)} "
                f"(CSV line {info.get('_csv_line', '?')})"
            )

        cwe = info.get("cwe")
        if cwe is not None and cwe not in VALID_CWES:
            warnings.append(
                f"L3 CWE {cwe} for key '{key}' is not in the known CWE set "
                f"(may be valid but not in our allowlist)"
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
    """Verify vuln-line/safe-line markers match vulnerable classification."""
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
    """Verify all benchmark categories are mapped in convert_theauditor.py."""
    if not BENCHMARK_PY.exists():
        warnings.append("L5 convert_theauditor.py not found - cannot verify scoring pipeline")
        return

    with open(BENCHMARK_PY, "r", encoding="utf-8") as f:
        scoring_content = f.read()

    mapped_categories = set()
    # Match category names in string literals (dict values, set members, comments)
    for m in re.finditer(r'"([a-z][a-z0-9_]+)"', scoring_content):
        mapped_categories.add(m.group(1))

    for cat in sorted(all_categories):
        if cat not in mapped_categories:
            warnings.append(
                f"L5 Category '{cat}' exists in ground truth but is not mapped "
                f"in convert_theauditor.py ADVERSARIAL_RULE_TO_CWE or EIDL_SIGNAL_TO_CWE"
            )


# ============================================================================
# Report
# ============================================================================
def print_report(csv_entries, annotations):
    """Print the full fidelity report."""
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
    print(f"Total TP:       {total_tp}")
    print(f"Total TN:       {total_tn}")
    if total > 0:
        print(f"TP/TN split:    {total_tp * 100.0 / total:.1f}% / {total_tn * 100.0 / total:.1f}%")
    print()

    print(f"{'Category':<24} {'CWE':>5} {'TP':>4} {'TN':>4} {'Total':>6} {'Balance':>8}")
    print("-" * 56)

    cat_cwes = {}
    for key, info in csv_entries.items():
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
        print(f"{cat:<24} {cwe:>5} {tp:>4} {tn:>4} {cat_total:>6} {balance:>8}")

    print("-" * 56)
    balance_str = f"{total_tp * 100 // total}/{total_tn * 100 // total}" if total > 0 else "N/A"
    print(f"{'TOTAL':<24} {'':>5} {total_tp:>4} {total_tn:>4} {total:>6} {balance_str:>8}")
    print()


def main():
    print("=" * 64)
    print("  Adversarial Evasion Benchmark Fidelity Validator v1.0")
    print("  Modeled after TheAuditor fidelity system (L1-L5)")
    print("=" * 64)
    print()

    # --- L1 ---
    print("[L1] Structural Integrity (CSV <-> annotation cross-reference)")
    csv_entries = parse_csv()
    annotations, vuln_lines, safe_lines, file_lines = scan_annotations()

    csv_keys = set(csv_entries.keys())
    ann_keys = set(annotations.keys())

    for key in sorted(csv_keys - ann_keys):
        errors.append(f"L1 Orphan CSV: '{key}' in ground truth but no annotation in source")
    for key in sorted(ann_keys - csv_keys):
        errors.append(f"L1 Orphan annotation: '{key}' in source but no CSV entry")

    l1_errors = len(errors)
    print(f"  Checks: duplicate keys, orphans, balanced start/end, unclosed snippets")
    print(f"  Result: {l1_errors} errors found")
    print()

    # --- L2 ---
    print("[L2] Roundtrip Fidelity (file existence, empty snippets)")
    check_roundtrip(csv_entries, annotations, file_lines)
    l2_errors = len(errors) - l1_errors
    print(f"  Checks: annotation source files exist on disk, snippets non-empty")
    print(f"  Result: {l2_errors} errors found")
    print()

    # --- L3 ---
    print("[L3] Schema Validation (required fields, valid CWEs, valid categories)")
    all_categories = check_schema(csv_entries)
    l3_errors = len(errors) - l1_errors - l2_errors
    print(f"  Checks: 4 CSV columns present, CWE in valid set, category in valid set")
    print(f"  Categories found: {len(all_categories)} ({', '.join(sorted(all_categories))})")
    print(f"  Result: {l3_errors} errors found")
    print()

    # --- L4 ---
    print("[L4] Semantic Fidelity (vuln-line/safe-line correctness, overlap detection)")
    check_semantics(csv_entries, annotations, vuln_lines, safe_lines)
    l4_errors = len(errors) - l1_errors - l2_errors - l3_errors
    print(f"  Checks: TP has vuln-line, TN has safe-line, markers inside snippet, no overlaps")
    print(f"  Result: {l4_errors} errors found")
    print()

    # --- L5 ---
    print("[L5] Scoring Pipeline Readiness (SIGNAL_MAP/RULE_MAP coverage)")
    check_scoring_pipeline(all_categories)
    l5_warnings = len(warnings)
    print(f"  Checks: every category has a mapping in convert_theauditor.py")
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
