#!/usr/bin/env python3
"""Go SAST Benchmark Fidelity Validator v2.0

Go uses filename-based test identity (like Java benchmark):
  CSV key: BenchmarkTest00001
  File: testcode/benchmark_test_00001.go

Fidelity levels:
  L1 -- Structural integrity (CSV <-> file cross-reference)
  L3 -- Schema validation (required fields, valid CWEs, valid categories)
  L5 -- Scoring pipeline readiness (CWE coverage in converter)

NOTE: L2 (roundtrip) and L4 (semantic/annotation) are N/A for Go's
filename-based architecture -- Go has no vuln-code-snippet annotations.

Exit 0 if all checks pass, 1 if any ERRORS, 2 if only WARNINGS.
No dependencies -- stdlib only.
"""

import os
import re
import sys
from collections import defaultdict
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
BENCH_ROOT = SCRIPT_DIR.parent
GO_DIR = BENCH_ROOT / "go"
CSV_FILE = GO_DIR / "expectedresults-0.3.2.csv"
CONVERTER_PY = SCRIPT_DIR / "convert_theauditor.py"
TESTCODE_DIR = GO_DIR / "testcode"

REQUIRED_FIELDS = {"key", "category", "cwe", "vulnerable"}
VALID_CWES = {
    20, 22, 78, 79, 89, 90, 94, 117, 287, 295, 327, 328, 330,
    352, 362, 434, 501, 502, 601, 614, 798, 862, 918, 943,
}

VALID_CATEGORIES = {
    "sqli", "cmdi", "pathtraver", "xss", "ssrf", "weakrand", "weakhash",
    "weakcipher", "securecookie", "redirect", "hardcodedcreds", "authnfailure",
    "tlsverify", "loginjection", "nosql", "authzfailure", "csrf", "codeinj",
    "ldapi", "trustbound", "deserial", "race_condition", "fileupload", "inputval",
}

errors = []
warnings = []


def csv_key_to_filename(key):
    """Convert BenchmarkTest00001 -> benchmark_test_00001.go"""
    s = re.sub(r"([A-Z])", r"_\1", key).lower().lstrip("_")
    s = re.sub(r"test(\d)", r"test_\1", s)
    return s + ".go"


def filename_to_csv_key(filename):
    """Convert benchmark_test_00001.go -> BenchmarkTest00001"""
    name = filename.replace(".go", "")
    parts = name.split("_")
    return "".join(p.capitalize() for p in parts)


# ============================================================================
# L1: Structural Integrity
# ============================================================================
def parse_csv():
    """Parse expectedresults CSV."""
    entries = {}
    seen_keys = {}

    if not CSV_FILE.exists():
        errors.append("L1 CSV file not found: %s" % CSV_FILE)
        return entries

    with open(CSV_FILE, "r", encoding="utf-8") as f:
        for line_num, line in enumerate(f, 1):
            stripped = line.strip()
            if not stripped or stripped.startswith("#"):
                continue
            parts = stripped.split(",")
            if len(parts) < 4:
                errors.append("L1 CSV line %d: expected 4 columns, got %d" % (line_num, len(parts)))
                continue

            key = parts[0].strip()
            category = parts[1].strip()
            vulnerable_str = parts[2].strip().lower()
            cwe_str = parts[3].strip()

            if key in seen_keys:
                errors.append(
                    "L1 CSV duplicate key '%s' at line %d "
                    "(first at line %d)" % (key, line_num, seen_keys[key])
                )
            seen_keys[key] = line_num

            try:
                cwe = int(cwe_str)
            except ValueError:
                errors.append("L3 Invalid CWE value '%s' for key '%s' at CSV line %d" % (cwe_str, key, line_num))
                cwe = -1

            entries[key] = {
                "key": key,
                "category": category,
                "vulnerable": vulnerable_str == "true",
                "cwe": cwe,
                "_csv_line": line_num,
            }

    return entries


def scan_test_files():
    """Scan testcode/ for benchmark_test_*.go files. Return set of CSV-style keys."""
    keys = set()
    if not TESTCODE_DIR.is_dir():
        errors.append("L1 testcode/ directory not found at %s" % TESTCODE_DIR)
        return keys

    for fn in os.listdir(TESTCODE_DIR):
        if fn.startswith("benchmark_test_") and fn.endswith(".go"):
            key = filename_to_csv_key(fn)
            keys.add(key)
    return keys


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
                "L3 Missing required fields for '%s': %s "
                "(CSV line %s)" % (key, sorted(missing), info.get("_csv_line", "?"))
            )

        cwe = info.get("cwe")
        if cwe is not None and cwe not in VALID_CWES:
            warnings.append(
                "L3 CWE %d for key '%s' is not in the known CWE set "
                "(may be valid but not in our allowlist)" % (cwe, key)
            )

        cat = info.get("category")
        if cat:
            all_categories.add(cat)
            if cat not in VALID_CATEGORIES:
                warnings.append(
                    "L3 Category '%s' for key '%s' is not in the expected category set" % (cat, key)
                )

    return all_categories


# ============================================================================
# L5: Scoring Pipeline Readiness
# ============================================================================
def check_scoring_pipeline(csv_entries):
    """Verify all CWEs in the benchmark have coverage in convert_theauditor.py."""
    if not CONVERTER_PY.exists():
        warnings.append("L5 convert_theauditor.py not found - cannot verify scoring pipeline")
        return

    with open(CONVERTER_PY, "r", encoding="utf-8") as f:
        converter_content = f.read()

    mapped_cwes = set()
    for m in re.finditer(r":\s*(\d+)", converter_content):
        mapped_cwes.add(int(m.group(1)))

    benchmark_cwes = set()
    for key, info in csv_entries.items():
        cwe = info.get("cwe")
        if cwe and cwe > 0:
            benchmark_cwes.add(cwe)

    for cwe in sorted(benchmark_cwes):
        if cwe not in mapped_cwes:
            warnings.append(
                "L5 CWE %d exists in ground truth but has no VULN_TYPE_TO_CWE "
                "mapping in convert_theauditor.py (taint flows for this CWE "
                "won't be converted)" % cwe
            )


# ============================================================================
# Report
# ============================================================================
def print_report(csv_entries, file_keys):
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

    print("CSV entries:    %d" % len(csv_entries))
    print("Test files:     %d" % len(file_keys))
    print("Match:          %s" % ("YES" if len(csv_entries) == len(file_keys) else "NO - MISMATCH"))
    print("Total TP:       %d" % total_tp)
    print("Total TN:       %d" % total_tn)
    if total > 0:
        print("TP/TN split:    %.1f%% / %.1f%%" % (total_tp * 100.0 / total, total_tn * 100.0 / total))
    print()

    cat_cwes = {}
    for key, info in csv_entries.items():
        cat = info.get("category")
        cwe = info.get("cwe")
        if cat and cwe:
            cat_cwes[cat] = cwe

    print("%-20s %5s %4s %4s %6s %8s" % ("Category", "CWE", "TP", "TN", "Total", "Balance"))
    print("-" * 52)
    for cat in sorted(categories.keys()):
        tp = categories[cat]["tp"]
        tn = categories[cat]["tn"]
        cat_total = tp + tn
        cwe = cat_cwes.get(cat, "?")
        if cat_total > 0:
            balance = "%d/%d" % (tp * 100 // cat_total, tn * 100 // cat_total)
        else:
            balance = "N/A"
        print("%-20s %5s %4d %4d %6d %8s" % (cat, cwe, tp, tn, cat_total, balance))

    print("-" * 52)
    balance_str = "%d/%d" % (total_tp * 100 // total, total_tn * 100 // total) if total > 0 else "N/A"
    print("%-20s %5s %4d %4d %6d %8s" % ("TOTAL", "", total_tp, total_tn, total, balance_str))
    print()


def main():
    print("=" * 64)
    print("  Go SAST Benchmark Fidelity Validator v2.0")
    print("  Filename-based matching (L1, L3, L5)")
    print("=" * 64)
    print()

    # --- L1: Structural Integrity ---
    print("[L1] Structural Integrity (CSV <-> file cross-reference)")
    csv_entries = parse_csv()
    file_keys = scan_test_files()

    csv_keys = set(csv_entries.keys())

    for key in sorted(csv_keys - file_keys):
        expected_file = csv_key_to_filename(key)
        errors.append("L1 Orphan CSV: '%s' -- no file testcode/%s" % (key, expected_file))
    for key in sorted(file_keys - csv_keys):
        errors.append("L1 Orphan file: '%s' -- .go file exists but no CSV entry" % key)

    l1_errors = len(errors)
    print("  Checks: duplicate keys, orphan CSV entries, orphan files")
    print("  Result: %d errors found" % l1_errors)
    print()

    # --- L3: Schema Validation ---
    print("[L3] Schema Validation (required fields, valid CWEs, valid categories)")
    all_categories = check_schema(csv_entries)
    l3_errors = len(errors) - l1_errors
    print("  Checks: 4 CSV columns present, CWE in valid set, category in valid set")
    print("  Categories found: %d (%s)" % (len(all_categories), ", ".join(sorted(all_categories))))
    print("  Result: %d errors found" % l3_errors)
    print()

    # --- L5: Scoring Pipeline Readiness ---
    print("[L5] Scoring Pipeline Readiness (CWE coverage in converter)")
    check_scoring_pipeline(csv_entries)
    l5_warnings = len(warnings)
    print("  Checks: every CWE has VULN_TYPE_TO_CWE mapping")
    print("  Result: %d warnings" % l5_warnings)
    print()

    print("NOTE: L2 (roundtrip) and L4 (semantic) are N/A for Go (filename-based, no annotations)")
    print()

    # --- Summary ---
    print("=" * 64)
    print("  SUMMARY")
    print("=" * 64)
    print()

    print_report(csv_entries, file_keys)

    if errors:
        print("ERRORS: %d" % len(errors))
        for err in errors[:20]:
            print("  [ERROR] %s" % err)
        if len(errors) > 20:
            print("  ... and %d more" % (len(errors) - 20))
        print()

    if warnings:
        print("WARNINGS: %d" % len(warnings))
        for warn in warnings:
            print("  [WARN]  %s" % warn)
        print()

    if errors:
        print("RESULT: FAIL")
        print("  %d errors must be fixed before benchmark is valid." % len(errors))
        return 1
    elif warnings:
        print("RESULT: PASS WITH WARNINGS")
        print("  %d warnings found. Review recommended but not blocking." % len(warnings))
        return 2
    else:
        print("RESULT: PASS")
        print("  All checks passed. Benchmark is valid.")
        return 0


if __name__ == "__main__":
    sys.exit(main())
