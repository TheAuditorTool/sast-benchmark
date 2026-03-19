#!/usr/bin/env python3
"""Validate Bash SAST benchmark consistency.

Checks:
1. Every YAML key has a matching annotation in source
2. Every annotation has a matching YAML entry
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
BASH_DIR = os.path.join(BENCH_ROOT, "bash")
YAML_FILE = os.path.join(BASH_DIR, "bash_ground_truth.yml")
SCAN_DIRS = [
    os.path.join(BASH_DIR, "original"),
    os.path.join(BASH_DIR, "adversarial"),
    os.path.join(BASH_DIR, "deepflow-webhook"),
    os.path.join(BASH_DIR, "deepflow-ops"),
    os.path.join(BASH_DIR, "dataforge"),
]

PAT_START = re.compile(r"vuln-code-snippet\s+start\s+(\S+)")
PAT_END = re.compile(r"vuln-code-snippet\s+end\s+(\S+)")

errors = []


def parse_yaml():
    """Parse bash_ground_truth.yml. Returns dict of {key: {category, vulnerable, cwe}}."""
    entries = {}
    current = None

    with open(YAML_FILE, "r", encoding="utf-8") as f:
        for line_num, line in enumerate(f, 1):
            stripped = line.strip()
            if not stripped or stripped.startswith("#"):
                continue

            if stripped.startswith("- key:"):
                if current:
                    entries[current["key"]] = current
                key = stripped.split(":", 1)[1].strip()
                current = {"key": key, "line": line_num}
                continue

            if current and ":" in stripped and not stripped.startswith("-"):
                k, _, v = stripped.partition(":")
                k = k.strip()
                v = v.strip().strip('"').strip("'")
                if k == "vulnerable":
                    current[k] = v.lower() == "true"
                elif k == "cwe":
                    current[k] = v
                elif k in ("file", "category", "description"):
                    current[k] = v

    if current:
        entries[current["key"]] = current

    # Check for duplicates (the dict silently overwrites — detect here)
    seen_keys = {}
    with open(YAML_FILE, "r", encoding="utf-8") as f:
        for line_num, line in enumerate(f, 1):
            stripped = line.strip()
            if stripped.startswith("- key:"):
                key = stripped.split(":", 1)[1].strip()
                if key in seen_keys:
                    errors.append(
                        f"YAML: duplicate key '{key}' at line {line_num} "
                        f"(first at line {seen_keys[key]})"
                    )
                seen_keys[key] = line_num

    return entries


def scan_annotations():
    """Scan .sh files for vuln-code-snippet annotations. Return {key: {file, start, end}}."""
    annotations = {}
    open_snippets = {}

    for scan_dir in SCAN_DIRS:
        if not os.path.isdir(scan_dir):
            continue
        for root, dirs, files in os.walk(scan_dir):
            dirs[:] = [d for d in dirs if d not in (".git", "node_modules")]
            for fn in sorted(files):
                if not fn.endswith(".sh"):
                    continue
                filepath = os.path.join(root, fn)
                rel = os.path.relpath(filepath, BASH_DIR).replace("\\", "/")
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

    for key, (file, line) in open_snippets.items():
        errors.append(f"Annotation: unclosed start for '{key}' in {file}:{line}")

    return annotations


def main():
    print("=" * 60)
    print("Bash SAST Benchmark Validation")
    print("=" * 60)
    print()

    yaml_entries = parse_yaml()
    annotations = scan_annotations()

    yaml_keys = set(yaml_entries.keys())
    ann_keys = set(annotations.keys())

    # Orphan checks
    yaml_only = yaml_keys - ann_keys
    ann_only = ann_keys - yaml_keys

    for key in sorted(yaml_only):
        errors.append(f"Orphan YAML: '{key}' in ground truth but no annotation in source")

    for key in sorted(ann_only):
        errors.append(f"Orphan annotation: '{key}' in source but no YAML entry")

    # Stats
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
    print(f"Total TP:       {total_tp}")
    print(f"Total TN:       {total_tn}")
    if total > 0:
        print(f"TP/TN ratio:    {total_tp * 100 // total}/{total_tn * 100 // total}")
    print()

    print(f"{'Category':<20} {'TP':>4} {'TN':>4} {'Total':>6} {'Balance':>8}")
    print("-" * 46)
    for cat in sorted(categories.keys()):
        tp = categories[cat]["tp"]
        tn = categories[cat]["tn"]
        cat_total = tp + tn
        balance = f"{tp * 100 // cat_total}/{tn * 100 // cat_total}" if cat_total > 0 else "N/A"
        print(f"{cat:<20} {tp:>4} {tn:>4} {cat_total:>6} {balance:>8}")
    print("-" * 46)
    balance_str = f"{total_tp * 100 // total}/{total_tn * 100 // total}" if total > 0 else "N/A"
    print(f"{'TOTAL':<20} {total_tp:>4} {total_tn:>4} {total:>6} {balance_str:>8}")
    print()

    # Report
    if errors:
        print(f"ERRORS: {len(errors)}")
        for err in errors:
            print(f"  - {err}")
        print()
        print("RESULT: FAIL")
        return 1
    else:
        print("RESULT: PASS")
        print("All checks passed. Ground truth and annotations are consistent.")
        return 0


if __name__ == "__main__":
    sys.exit(main())
