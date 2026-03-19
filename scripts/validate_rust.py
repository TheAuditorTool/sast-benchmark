#!/usr/bin/env python3
"""Validate Rust SAST benchmark consistency.

Checks:
1. Every CSV key has a matching annotation in source
2. Every annotation has a matching CSV entry
3. Start/end pairs are balanced (no unclosed snippets)
4. Per-category TP/TN stats
5. No duplicate keys

Exit 0 if all checks pass, 1 if any fail.
No dependencies — stdlib only.
"""

import os
import re
import sys
from collections import defaultdict

# Paths relative to this script
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
BENCH_ROOT = os.path.dirname(SCRIPT_DIR)
RUST_DIR = os.path.join(BENCH_ROOT, "rust")
CSV_FILE = os.path.join(RUST_DIR, "expectedresults-0.1.csv")
SCAN_DIRS = [
    os.path.join(RUST_DIR, "apps"),
    os.path.join(RUST_DIR, "testcode"),
]

PAT_START = re.compile(r"vuln-code-snippet\s+start\s+(\S+)")
PAT_END = re.compile(r"vuln-code-snippet\s+end\s+(\S+)")

errors = []


def parse_csv():
    """Parse CSV answer key, return dict of {key: {category, vulnerable, cwe}}."""
    entries = {}
    with open(CSV_FILE, "r", encoding="utf-8") as f:
        for line_num, line in enumerate(f, 1):
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            parts = line.split(",")
            if len(parts) != 4:
                errors.append(f"CSV line {line_num}: expected 4 fields, got {len(parts)}: {line}")
                continue
            key, category, vulnerable, cwe = parts
            if key in entries:
                errors.append(f"CSV: duplicate key '{key}' at line {line_num}")
            entries[key] = {
                "category": category,
                "vulnerable": vulnerable.lower() == "true",
                "cwe": cwe,
                "line": line_num,
            }
    return entries


def scan_annotations():
    """Scan .rs files for vuln-code-snippet annotations. Return {key: {file, start, end}}."""
    annotations = {}
    open_snippets = {}  # key -> (file, start_line)

    for scan_dir in SCAN_DIRS:
        if not os.path.isdir(scan_dir):
            continue
        for root, dirs, files in os.walk(scan_dir):
            dirs[:] = [d for d in dirs if d not in ("target", ".git", "node_modules")]
            for fn in sorted(files):
                if not fn.endswith(".rs"):
                    continue
                filepath = os.path.join(root, fn)
                rel = os.path.relpath(filepath, RUST_DIR).replace("\\", "/")
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
                                f"Annotation: duplicate start for '{key}' in {rel}:{i} "
                                f"(first opened at {open_snippets[key][0]}:{open_snippets[key][1]})"
                            )
                        open_snippets[key] = (rel, i)

                    m = PAT_END.search(line)
                    if m:
                        key = m.group(1)
                        if key in open_snippets:
                            start_file, start_line = open_snippets.pop(key)
                            if key in annotations:
                                errors.append(f"Annotation: duplicate key '{key}' in {rel}:{i}")
                            annotations[key] = {
                                "file": start_file,
                                "start": start_line,
                                "end": i,
                            }
                        else:
                            errors.append(f"Annotation: end without start for '{key}' in {rel}:{i}")

    # Check for unclosed snippets
    for key, (file, line) in open_snippets.items():
        errors.append(f"Annotation: unclosed start for '{key}' in {file}:{line}")

    return annotations


def main():
    print("=" * 60)
    print("Rust SAST Benchmark Validation")
    print("=" * 60)
    print()

    # Parse
    csv_entries = parse_csv()
    annotations = scan_annotations()

    csv_keys = set(csv_entries.keys())
    ann_keys = set(annotations.keys())

    # Check orphans
    csv_only = csv_keys - ann_keys
    ann_only = ann_keys - csv_keys

    for key in sorted(csv_only):
        errors.append(f"Orphan CSV: '{key}' in CSV but no annotation in source")

    for key in sorted(ann_only):
        errors.append(f"Orphan annotation: '{key}' in source but no CSV entry")

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

    print(f"CSV entries:    {len(csv_entries)}")
    print(f"Annotations:    {len(annotations)}")
    print(f"Total TP:       {total_tp}")
    print(f"Total TN:       {total_tn}")
    print(f"TP/TN ratio:    {total_tp * 100 // (total_tp + total_tn)}/{total_tn * 100 // (total_tp + total_tn)}")
    print()

    print(f"{'Category':<16} {'TP':>4} {'TN':>4} {'Total':>6} {'Balance':>8}")
    print("-" * 42)
    for cat in sorted(categories.keys()):
        tp = categories[cat]["tp"]
        tn = categories[cat]["tn"]
        total = tp + tn
        balance = f"{tp * 100 // total}/{tn * 100 // total}" if total > 0 else "N/A"
        print(f"{cat:<16} {tp:>4} {tn:>4} {total:>6} {balance:>8}")
    print("-" * 42)
    print(f"{'TOTAL':<16} {total_tp:>4} {total_tn:>4} {total_tp + total_tn:>6}")
    print()

    # Report
    if errors:
        print(f"ERRORS: {len(errors)}")
        print()
        for err in errors:
            print(f"  ERROR: {err}")
        print()
        print("RESULT: FAIL")
        return 1
    else:
        print("RESULT: PASS (all checks passed)")
        return 0


if __name__ == "__main__":
    sys.exit(main())
