#!/usr/bin/env python3
"""Ruby SAST Benchmark Fidelity Validator v2.0

Filename-based validation (Go/Java model). No annotations.

  L1 -- Structural integrity (CSV <-> file cross-reference)
  L2 -- File existence (every CSV key has a matching .rb file)
  L3 -- Schema validation (required fields, valid CWEs, valid categories)
  L4 -- Balance check (TP/TN per category)
  L5 -- Scoring pipeline readiness (CWE coverage in converter)

Exit 0 if all checks pass, 1 if any ERRORS, 2 if only WARNINGS.
No dependencies -- stdlib only.
"""

import hashlib
import json
import re
import sys
from collections import defaultdict
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
BENCH_ROOT = SCRIPT_DIR.parent
RUBY_DIR = BENCH_ROOT / "ruby"
CSV_FILE = RUBY_DIR / "expectedresults-0.3.1.csv"
TESTCODE_DIR = RUBY_DIR / "testcode"
CONVERTER_PY = SCRIPT_DIR / "convert_theauditor.py"

VALID_CWES = {
    22, 78, 79, 89, 90, 94, 98, 113, 117, 287, 327, 328, 330,
    352, 434, 470, 502, 601, 611, 614, 798,
    862, 915, 918, 1333, 1336,
}

VALID_CATEGORIES = {
    "sqli", "xss", "cmdi", "fileinclusion", "pathtraver", "deserial",
    "ssrf", "codeinj", "massassign", "unsafereflect", "ssti",
    "dynmethod", "redirect", "xxe", "fileupload", "weakhash",
    "weakrand", "weakcipher", "hardcodedcreds", "csrf", "headerinj",
    "ldapi", "securecookie", "regex", "loginjection",
    "authnfailure", "authzfailure",
}

errors = []
warnings = []


def parse_csv():
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


def check_files(csv_entries):
    csv_keys = set(csv_entries.keys())
    rb_files = set()

    if not TESTCODE_DIR.is_dir():
        errors.append("L1 testcode directory not found: %s" % TESTCODE_DIR)
        return

    for f in TESTCODE_DIR.glob("benchmark_test_*.rb"):
        m = re.match(r"benchmark_test_(\d{5})\.rb$", f.name)
        if m:
            rb_files.add("BenchmarkTest" + m.group(1))

    for key in sorted(csv_keys - rb_files):
        errors.append("L2 CSV key '%s' has no matching file in testcode/" % key)

    for key in sorted(rb_files - csv_keys):
        errors.append("L2 File for '%s' exists in testcode/ but no CSV entry" % key)


def check_no_comments():
    count = 0
    for f in TESTCODE_DIR.glob("benchmark_test_*.rb"):
        with open(f, "r", encoding="utf-8", errors="replace") as fh:
            for i, line in enumerate(fh, 1):
                stripped = line.lstrip()
                if stripped.startswith("#"):
                    if stripped.startswith("#!") and i == 1:
                        continue
                    if stripped.startswith("# frozen_string_literal:"):
                        continue
                    if stripped.startswith("# encoding:"):
                        continue
                    errors.append("L2 Comment found in %s:%d: %s" % (f.name, i, stripped.rstrip()))
                    count += 1
    return count


def check_schema(csv_entries):
    all_categories = set()

    for key, info in csv_entries.items():
        cwe = info.get("cwe")
        if cwe is not None and cwe not in VALID_CWES:
            warnings.append(
                "L3 CWE %d for key '%s' is not in the known CWE set" % (cwe, key)
            )

        cat = info.get("category")
        if cat:
            all_categories.add(cat)
            if cat not in VALID_CATEGORIES:
                warnings.append(
                    "L3 Category '%s' for key '%s' is not in the expected category set" % (cat, key)
                )

        if not re.match(r"BenchmarkTest\d{5}$", key):
            errors.append("L3 Key '%s' does not match BenchmarkTestNNNNN format" % key)

    return all_categories


def check_balance(csv_entries):
    categories = defaultdict(lambda: {"tp": 0, "tn": 0})
    for info in csv_entries.values():
        cat = info.get("category", "unknown")
        if info.get("vulnerable"):
            categories[cat]["tp"] += 1
        else:
            categories[cat]["tn"] += 1

    for cat, counts in sorted(categories.items()):
        if counts["tp"] != counts["tn"]:
            errors.append(
                "L4 Category '%s' is unbalanced: %d TP / %d TN" % (cat, counts["tp"], counts["tn"])
            )

    return categories


def check_scoring_pipeline(csv_entries):
    if not CONVERTER_PY.exists():
        warnings.append("L5 convert_theauditor.py not found - cannot verify scoring pipeline")
        return

    with open(CONVERTER_PY, "r", encoding="utf-8") as f:
        converter_content = f.read()

    mapped_cwes = set()
    for m in re.finditer(r":\s*(\d+)", converter_content):
        mapped_cwes.add(int(m.group(1)))

    benchmark_cwes = set()
    for info in csv_entries.values():
        cwe = info.get("cwe")
        if cwe and cwe > 0:
            benchmark_cwes.add(cwe)

    for cwe in sorted(benchmark_cwes):
        if cwe not in mapped_cwes:
            warnings.append(
                "L5 CWE %d exists in ground truth but has no mapping in convert_theauditor.py" % cwe
            )


def print_report(csv_entries, categories):
    total_tp = sum(c["tp"] for c in categories.values())
    total_tn = sum(c["tn"] for c in categories.values())
    total = total_tp + total_tn

    print("CSV entries:    %d" % len(csv_entries))
    print("Test files:     %d" % len(list(TESTCODE_DIR.glob("benchmark_test_*.rb"))))
    print("Total TP:       %d" % total_tp)
    print("Total TN:       %d" % total_tn)
    if total > 0:
        print("TP/TN split:    %.1f%% / %.1f%%" % (total_tp * 100.0 / total, total_tn * 100.0 / total))
    print()

    cat_cwes = {}
    for info in csv_entries.values():
        cat = info.get("category")
        cwe = info.get("cwe")
        if cat and cwe:
            cat_cwes[cat] = cwe

    print("%-20s %5s %4s %4s %6s" % ("Category", "CWE", "TP", "TN", "Total"))
    print("-" * 42)

    for cat in sorted(categories.keys()):
        tp = categories[cat]["tp"]
        tn = categories[cat]["tn"]
        cwe = cat_cwes.get(cat, "?")
        print("%-20s %5s %4d %4d %6d" % (cat, cwe, tp, tn, tp + tn))

    print("-" * 42)
    print("%-20s %5s %4d %4d %6d" % ("TOTAL", "", total_tp, total_tn, total))
    print()


def main():
    print("=" * 64)
    print("  Ruby SAST Benchmark Fidelity Validator v2.0")
    print("  Filename-based validation (Go/Java model)")
    print("=" * 64)
    print()

    print("[L1] Structural Integrity (CSV parsing)")
    csv_entries = parse_csv()
    l1_errors = len(errors)
    print("  CSV entries: %d" % len(csv_entries))
    print("  Result: %d errors" % l1_errors)
    print()

    print("[L2] File Existence + Zero Comments")
    check_files(csv_entries)
    comment_count = check_no_comments()
    l2_errors = len(errors) - l1_errors
    print("  Comment lines found: %d" % comment_count)
    print("  Result: %d errors" % l2_errors)
    print()

    print("[L3] Schema Validation (key format, CWEs, categories)")
    all_categories = check_schema(csv_entries)
    l3_errors = len(errors) - l1_errors - l2_errors
    print("  Categories: %d (%s)" % (len(all_categories), ", ".join(sorted(all_categories))))
    print("  Result: %d errors" % l3_errors)
    print()

    print("[L4] Balance Check (TP/TN per category)")
    categories = check_balance(csv_entries)
    l4_errors = len(errors) - l1_errors - l2_errors - l3_errors
    print("  Result: %d errors" % l4_errors)
    print()

    print("[L5] Scoring Pipeline Readiness")
    check_scoring_pipeline(csv_entries)
    l5_warnings = len(warnings)
    print("  Result: %d warnings" % l5_warnings)
    print()

    print("[L6] SARIF Integrity")
    sarif_path = RUBY_DIR / "theauditor.sarif"
    if not sarif_path.exists():
        print("  theauditor.sarif not found -- skipping")
    else:
        try:
            with open(sarif_path, "r", encoding="utf-8") as sf:
                sarif_data = json.load(sf)
            integrity = sarif_data.get("runs", [{}])[0].get("properties", {}).get("integrity")
            if integrity is None:
                warnings.append("L6 theauditor.sarif has no integrity metadata")
            else:
                h = hashlib.sha256()
                with open(CSV_FILE, "rb") as cf:
                    for chunk in iter(lambda: cf.read(65536), b""):
                        h.update(chunk)
                if h.hexdigest() != integrity.get("csv_sha256"):
                    warnings.append("L6 theauditor.sarif is STALE: CSV hash mismatch")
                else:
                    print("  SARIF integrity: CURRENT")
        except (json.JSONDecodeError, OSError) as e:
            warnings.append("L6 Could not read theauditor.sarif: %s" % e)
    l6_warnings = len(warnings) - l5_warnings
    print("  Result: %d warnings" % l6_warnings)
    print()

    print("=" * 64)
    print("  SUMMARY")
    print("=" * 64)
    print()

    print_report(csv_entries, categories)

    if errors:
        print("ERRORS: %d" % len(errors))
        for err in errors:
            print("  [ERROR] %s" % err)
        print()

    if warnings:
        print("WARNINGS: %d" % len(warnings))
        for warn in warnings:
            print("  [WARN]  %s" % warn)
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
