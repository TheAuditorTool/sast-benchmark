#!/usr/bin/env python3
"""Validate Go SAST benchmark consistency.

Go uses filename-based test identity (like Java benchmark):
  CSV key: BenchmarkTest00001
  File: testcode/benchmark_test_00001.go

Checks:
1. Every CSV key has a matching .go file in testcode/
2. Every benchmark_test_*.go file has a matching CSV entry
3. Per-category TP/TN stats
4. No duplicate keys

Exit 0 if all checks pass, 1 if any fail.
No dependencies — stdlib only.
"""

import os
import re
import sys
from collections import defaultdict

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
BENCH_ROOT = os.path.dirname(SCRIPT_DIR)
GO_DIR = os.path.join(BENCH_ROOT, "go")
CSV_FILE = os.path.join(GO_DIR, "expectedresults-0.1.csv")
TESTCODE_DIR = os.path.join(GO_DIR, "testcode")

errors = []


def csv_key_to_filename(key):
    """Convert BenchmarkTest00001 -> benchmark_test_00001.go"""
    # BenchmarkTest00001 -> Benchmark_Test_00001 -> benchmark_test_00001
    s = re.sub(r"([A-Z])", r"_\1", key).lower().lstrip("_")
    # benchmark_test00001 -> benchmark_test_00001
    s = re.sub(r"test(\d)", r"test_\1", s)
    return s + ".go"


def filename_to_csv_key(filename):
    """Convert benchmark_test_00001.go -> BenchmarkTest00001"""
    name = filename.replace(".go", "")
    parts = name.split("_")
    return "".join(p.capitalize() for p in parts)


def parse_csv():
    entries = {}
    with open(CSV_FILE, "r", encoding="utf-8") as f:
        for line_num, line in enumerate(f, 1):
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            parts = line.split(",")
            if len(parts) != 4:
                errors.append(f"CSV line {line_num}: expected 4 fields, got {len(parts)}")
                continue
            key = parts[0]
            if key in entries:
                errors.append(f"CSV: duplicate key '{key}' at line {line_num}")
            entries[key] = {
                "category": parts[1],
                "vulnerable": parts[2].lower() == "true",
                "cwe": parts[3],
            }
    return entries


def scan_test_files():
    """Scan testcode/ for benchmark_test_*.go files. Return set of CSV-style keys."""
    keys = set()
    if not os.path.isdir(TESTCODE_DIR):
        errors.append(f"testcode/ directory not found at {TESTCODE_DIR}")
        return keys

    for fn in os.listdir(TESTCODE_DIR):
        if fn.startswith("benchmark_test_") and fn.endswith(".go"):
            key = filename_to_csv_key(fn)
            keys.add(key)
    return keys


def main():
    print("=" * 60)
    print("Go SAST Benchmark Validation")
    print("=" * 60)
    print()

    csv_entries = parse_csv()
    file_keys = scan_test_files()

    csv_keys = set(csv_entries.keys())

    csv_only = csv_keys - file_keys
    file_only = file_keys - csv_keys

    for key in sorted(csv_only):
        expected_file = csv_key_to_filename(key)
        errors.append(f"Orphan CSV: '{key}' — no file testcode/{expected_file}")

    for key in sorted(file_only):
        errors.append(f"Orphan file: '{key}' — .go file exists but no CSV entry")

    # Stats
    categories = defaultdict(lambda: {"tp": 0, "tn": 0})
    for key, info in csv_entries.items():
        cat = info["category"]
        if info["vulnerable"]:
            categories[cat]["tp"] += 1
        else:
            categories[cat]["tn"] += 1

    total_tp = sum(c["tp"] for c in categories.values())
    total_tn = sum(c["tn"] for c in categories.values())
    total = total_tp + total_tn

    print(f"CSV entries:    {len(csv_entries)}")
    print(f"Test files:     {len(file_keys)}")
    print(f"Total TP:       {total_tp}")
    print(f"Total TN:       {total_tn}")
    if total > 0:
        print(f"TP/TN ratio:    {total_tp * 100 // total}/{total_tn * 100 // total}")
    print()

    print(f"{'Category':<16} {'TP':>4} {'TN':>4} {'Total':>6}")
    print("-" * 34)
    for cat in sorted(categories.keys()):
        tp = categories[cat]["tp"]
        tn = categories[cat]["tn"]
        print(f"{cat:<16} {tp:>4} {tn:>4} {tp + tn:>6}")
    print("-" * 34)
    print(f"{'TOTAL':<16} {total_tp:>4} {total_tn:>4} {total:>6}")
    print()

    if errors:
        print(f"ERRORS: {len(errors)}")
        for err in errors[:20]:
            print(f"  ERROR: {err}")
        if len(errors) > 20:
            print(f"  ... and {len(errors) - 20} more")
        print()
        print("RESULT: FAIL")
        return 1
    else:
        print("RESULT: PASS (all checks passed)")
        return 0


if __name__ == "__main__":
    sys.exit(main())
