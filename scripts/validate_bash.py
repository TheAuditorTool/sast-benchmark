#!/usr/bin/env python3
"""Bash SAST Benchmark Fidelity Validator v3.0

1-file-1-test architecture (v0.5.1+).
Each benchmark_test_NNNNN.sh in testcode/ is one test case.
Ground truth is expectedresults-0.5.1.csv.
No annotation markers in source files.

Levels:
  L1 — Structural integrity: every CSV key has a corresponding .sh file
  L2 — Completeness: every .sh file in testcode/ has a CSV entry (no orphans)
  L3 — Schema validation: required fields, valid CWEs, valid categories
  L4 — Balance: per-category TP/TN counts, minimum 25/25 floor
  L5 — Scoring pipeline readiness: bash_benchmark.py category coverage

Exit 0 = all pass, 1 = errors, 2 = warnings only.
"""

import os
import re
import sys
from collections import defaultdict
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
BENCH_ROOT = SCRIPT_DIR.parent
BASH_DIR = BENCH_ROOT / "bash"
CSV_FILE = BASH_DIR / "expectedresults-0.5.2.csv"
TESTCODE_DIR = BASH_DIR / "testcode"
BENCHMARK_PY = BASH_DIR / "bash_benchmark.py"

VALID_CWES = {
    20, 22, 77, 78, 79, 88, 89, 90, 93, 94, 117, 119, 190, 200, 250, 269,
    276, 287, 295, 306, 319, 327, 328, 330, 352, 362, 367, 377, 434, 494,
    501, 502, 532, 601, 611, 614, 643, 732, 770, 798, 862, 863, 918, 943,
    1333,
}

MIN_PER_CATEGORY = 25

errors = []
warnings = []


# ============================================================================
# CSV parser
# ============================================================================
def parse_csv():
    entries = {}
    seen_keys = {}
    with open(CSV_FILE, "r", encoding="utf-8") as f:
        for line_num, line in enumerate(f, 1):
            s = line.strip()
            if not s or s.startswith("#"):
                continue
            parts = s.split(",")
            if len(parts) < 4:
                errors.append(f"L1 CSV line {line_num}: expected 4 columns, got {len(parts)}")
                continue
            key = parts[0].strip()
            category = parts[1].strip()
            vulnerable_str = parts[2].strip().lower()
            cwe_str = parts[3].strip()

            if key in seen_keys:
                errors.append(
                    f"L1 Duplicate CSV key '{key}' at line {line_num} "
                    f"(first at line {seen_keys[key]})"
                )
            seen_keys[key] = line_num

            try:
                cwe = int(cwe_str)
            except ValueError:
                errors.append(f"L3 Invalid CWE '{cwe_str}' for key '{key}' at CSV line {line_num}")
                cwe = -1

            entries[key] = {
                "key": key,
                "category": category,
                "vulnerable": vulnerable_str == "true",
                "cwe": cwe,
                "_line": line_num,
            }
    return entries


# ============================================================================
# L1: Every CSV key has a testcode file
# ============================================================================
def check_csv_has_files(csv_entries):
    missing = []
    for key in csv_entries:
        if not key.startswith("benchmark_test_"):
            continue
        src = TESTCODE_DIR / (key + ".sh")
        if not src.exists():
            missing.append(key)
    if missing:
        for k in missing:
            errors.append(f"L1 CSV key '{k}' has no file testcode/{k}.sh")
    return len(missing) == 0


# ============================================================================
# L2: Every testcode file has a CSV entry (no orphan files)
# ============================================================================
def check_files_have_csv(csv_entries):
    orphans = []
    if not TESTCODE_DIR.is_dir():
        errors.append(f"L2 testcode/ directory not found at {TESTCODE_DIR}")
        return
    for sh_file in sorted(TESTCODE_DIR.glob("benchmark_test_*.sh")):
        stem = sh_file.stem
        if stem not in csv_entries:
            orphans.append(sh_file.name)
    if orphans:
        for fn in orphans:
            errors.append(f"L2 Orphan file testcode/{fn} has no CSV entry")
    return len(orphans) == 0


# ============================================================================
# L3: Schema validation
# ============================================================================
def check_schema(csv_entries):
    all_categories = set()
    for key, info in csv_entries.items():
        cwe = info.get("cwe", -1)
        if cwe > 0 and cwe not in VALID_CWES:
            warnings.append(
                f"L3 CWE {cwe} for key '{key}' not in known CWE allowlist "
                f"(line {info.get('_line', '?')})"
            )
        cat = info.get("category", "")
        if not cat:
            errors.append(f"L3 Key '{key}' has empty category")
        else:
            all_categories.add(cat)
    return all_categories


# ============================================================================
# L4: Balance check
# ============================================================================
def check_balance(csv_entries):
    cat_counts = defaultdict(lambda: {"tp": 0, "tn": 0})
    for key, info in csv_entries.items():
        cat = info.get("category", "unknown")
        if info.get("vulnerable", False):
            cat_counts[cat]["tp"] += 1
        else:
            cat_counts[cat]["tn"] += 1

    for cat, counts in sorted(cat_counts.items()):
        t, s = counts["tp"], counts["tn"]
        if t < MIN_PER_CATEGORY:
            errors.append(
                f"L4 Category '{cat}' has only {t} TP (minimum {MIN_PER_CATEGORY})"
            )
        if s < MIN_PER_CATEGORY:
            errors.append(
                f"L4 Category '{cat}' has only {s} TN (minimum {MIN_PER_CATEGORY})"
            )

    total_tp = sum(c["tp"] for c in cat_counts.values())
    total_tn = sum(c["tn"] for c in cat_counts.values())
    if total_tp != total_tn:
        warnings.append(
            f"L4 Global TP/TN imbalance: {total_tp} TP vs {total_tn} TN "
            f"(ideal: exact 50/50)"
        )

    return cat_counts


# ============================================================================
# L5: Scoring pipeline readiness
# ============================================================================
def check_scoring_pipeline(all_categories):
    if not BENCHMARK_PY.exists():
        warnings.append("L5 bash_benchmark.py not found")
        return
    content = BENCHMARK_PY.read_text(encoding="utf-8")
    mapped_cats = set(re.findall(r':\s*"([a-z_]+)"', content))
    for cat in sorted(all_categories):
        if cat not in mapped_cats:
            warnings.append(
                f"L5 Category '{cat}' not mapped in bash_benchmark.py RULE_MAP/SINK_MAP"
            )


# ============================================================================
# Report
# ============================================================================
def print_report(csv_entries, cat_counts):
    total_tp = sum(c["tp"] for c in cat_counts.values())
    total_tn = sum(c["tn"] for c in cat_counts.values())
    total = total_tp + total_tn

    cat_cwes = {}
    for key, info in csv_entries.items():
        cat = info.get("category")
        cwe = info.get("cwe")
        if cat and cwe:
            cat_cwes[cat] = cwe

    print(f"CSV entries:    {len(csv_entries)}")
    print(f"Testcode files: {len(list(TESTCODE_DIR.glob('benchmark_test_*.sh')))}")
    print(f"Total TP:       {total_tp}")
    print(f"Total TN:       {total_tn}")
    if total > 0:
        print(f"TP/TN split:    {total_tp * 100.0 / total:.1f}% / {total_tn * 100.0 / total:.1f}%")
    print()

    print(f"{'Category':<25} {'CWE':>5} {'TP':>4} {'TN':>4} {'Total':>6}")
    print("-" * 48)
    for cat in sorted(cat_counts):
        tp = cat_counts[cat]["tp"]
        tn = cat_counts[cat]["tn"]
        cwe = cat_cwes.get(cat, "?")
        print(f"{cat:<25} {cwe:>5} {tp:>4} {tn:>4} {tp + tn:>6}")
    print("-" * 48)
    print(f"{'TOTAL':<25} {'':>5} {total_tp:>4} {total_tn:>4} {total:>6}")
    print()


def main():
    print("=" * 64)
    print("  Bash SAST Benchmark Fidelity Validator v3.0")
    print("  1-file-1-test architecture (v0.5.1+)")
    print("=" * 64)
    print()

    # L1: CSV -> files
    print("[L1] CSV entries have corresponding testcode/ files")
    csv_entries = parse_csv()
    check_csv_has_files(csv_entries)
    l1_errors = len(errors)
    print(f"  CSV entries: {len(csv_entries)}")
    print(f"  Result: {l1_errors} errors")
    print()

    # L2: files -> CSV
    print("[L2] testcode/ files have CSV entries (no orphans)")
    check_files_have_csv(csv_entries)
    l2_errors = len(errors) - l1_errors
    print(f"  Result: {l2_errors} errors")
    print()

    # L3: Schema
    print("[L3] Schema validation (CWEs, categories)")
    all_categories = check_schema(csv_entries)
    l3_errors = len(errors) - l1_errors - l2_errors
    print(f"  Categories found: {len(all_categories)} ({', '.join(sorted(all_categories))})")
    print(f"  Result: {l3_errors} errors")
    print()

    # L4: Balance
    print(f"[L4] Per-category balance (minimum {MIN_PER_CATEGORY} TP and {MIN_PER_CATEGORY} TN)")
    cat_counts = check_balance(csv_entries)
    l4_errors = len(errors) - l1_errors - l2_errors - l3_errors
    print(f"  Result: {l4_errors} errors")
    print()

    # L5: Pipeline
    print("[L5] Scoring pipeline readiness (bash_benchmark.py coverage)")
    check_scoring_pipeline(all_categories)
    l5_warnings = len(warnings)
    print(f"  Result: {l5_warnings} warnings")
    print()

    print("=" * 64)
    print("  SUMMARY")
    print("=" * 64)
    print()

    print_report(csv_entries, cat_counts)

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
        print(f"  {len(warnings)} warnings. Review recommended but not blocking.")
        return 2
    else:
        print("RESULT: PASS")
        print("  All checks passed. Benchmark is valid.")
        return 0


if __name__ == "__main__":
    sys.exit(main())
