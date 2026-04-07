#!/usr/bin/env python3
"""Migrate PHP testcode to eliminate target leakage.

Strips annotations/comments, renames files+functions to generic
benchmark_test_NNNNN.php / benchmarkTestNNNNN() pattern.

Deterministic (random.seed(20260407)), non-destructive.
Outputs to testcode/migration/ -- originals untouched.
"""

import json
import random
import re
import sys
from pathlib import Path

SEED = 20260407
SCRIPT_DIR = Path(__file__).parent
REPO_DIR = SCRIPT_DIR.parent
PHP_DIR = REPO_DIR / "php"
TESTCODE_DIR = PHP_DIR / "testcode"
MIGRATION_DIR = TESTCODE_DIR / "migration"
OLD_CSV = PHP_DIR / "expectedresults-0.3.0.csv"
NEW_CSV = PHP_DIR / "expectedresults-0.3.1.csv"
RENAME_MAP_FILE = PHP_DIR / "rename_map.json"


def extract_annotation_key(filepath):
    with open(filepath, "r", encoding="utf-8") as f:
        for line in f:
            m = re.search(r"vuln-code-snippet\s+start\s+(\S+)", line)
            if m:
                return m.group(1)
    return None


def strip_comments(content):
    lines = content.split("\n")
    result = []

    for line in lines:
        stripped = line.strip()

        if re.match(r"//\s*vuln-code-snippet\s+(start|end)\s+", stripped):
            continue
        if re.match(r"//\s*Legacy PHP\s+", stripped):
            continue
        if re.match(r"//\s*Simulates\s+(Blade|Twig)\s+", stripped):
            continue
        if re.match(r"//\s*Re-establish\s+DB\s+", stripped):
            continue

        line = re.sub(r"\s*//\s*vuln-code-snippet\s+(vuln-line|safe-line)\s+\S+.*$", "", line)
        line = re.sub(r"\s*//\s*Legacy PHP\s+.*$", "", line)

        result.append(line.rstrip())

    # Collapse 3+ consecutive blank lines to 2
    collapsed = []
    blank_count = 0
    for line in result:
        if line == "":
            blank_count += 1
            if blank_count <= 2:
                collapsed.append(line)
        else:
            blank_count = 0
            collapsed.append(line)

    while collapsed and collapsed[-1] == "":
        collapsed.pop()

    return "\n".join(collapsed) + "\n"


def rename_entry_function(content, new_name):
    return re.sub(
        r"^(function\s+)\w+(\([^)]*BenchmarkRequest\b)",
        rf"\g<1>{new_name}\2",
        content,
        count=1,
        flags=re.MULTILINE,
    )


def strip_helper_numbers(content):
    return re.sub(
        r"^(function\s+(?:createUser|processId|formatOutput))\d+(\()",
        r"\1\2",
        content,
        flags=re.MULTILINE,
    )


def load_csv(csv_path):
    entries = []
    with open(csv_path, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            parts = line.split(",")
            if len(parts) >= 4:
                entries.append(tuple(p.strip() for p in parts[:4]))
    return entries


def main():
    test_files = sorted(
        f.name for f in TESTCODE_DIR.glob("*.php") if f.name != "shared.php"
    )
    print(f"Found {len(test_files)} test files")

    file_to_key = {}
    for fname in test_files:
        key = extract_annotation_key(TESTCODE_DIR / fname)
        if key is None:
            print(f"ERROR: No annotation key in {fname}", file=sys.stderr)
            sys.exit(1)
        file_to_key[fname] = key
    print(f"Extracted {len(file_to_key)} annotation keys")

    csv_entries = load_csv(OLD_CSV)
    testcode_entries = {}
    for key, cat, vuln, cwe in csv_entries:
        if key.startswith("php_"):
            testcode_entries[key] = (cat, vuln, cwe)

    print(f"CSV testcode entries: {len(testcode_entries)}")

    missing = set(file_to_key.values()) - set(testcode_entries.keys())
    if missing:
        print(f"ERROR: annotation keys not in CSV: {missing}", file=sys.stderr)
        sys.exit(1)

    orphans = set(testcode_entries.keys()) - set(file_to_key.values())
    if orphans:
        print(f"ERROR: CSV keys with no file: {orphans}", file=sys.stderr)
        sys.exit(1)

    random.seed(SEED)
    shuffled = test_files[:]
    random.shuffle(shuffled)

    file_to_number = {}
    for idx, fname in enumerate(shuffled, 1):
        file_to_number[fname] = idx

    MIGRATION_DIR.mkdir(exist_ok=True)

    rename_map = {}
    key_map = {}

    for fname in test_files:
        num = file_to_number[fname]
        new_fname = f"benchmark_test_{num:05d}.php"
        new_func = f"benchmarkTest{num:05d}"
        new_key = f"BenchmarkTest{num:05d}"
        old_key = file_to_key[fname]

        content = (TESTCODE_DIR / fname).read_text(encoding="utf-8")
        content = strip_comments(content)
        content = rename_entry_function(content, new_func)
        content = strip_helper_numbers(content)

        (MIGRATION_DIR / new_fname).write_text(content, encoding="utf-8")

        rename_map[fname] = new_fname
        key_map[old_key] = new_key

    print(f"Wrote {len(rename_map)} files to migration/")

    sorted_entries = []
    for old_key, (cat, vuln, cwe) in testcode_entries.items():
        sorted_entries.append((key_map[old_key], cat, vuln, cwe))
    sorted_entries.sort()

    tp = sum(1 for _, _, v, _ in sorted_entries if v == "true")
    tn = sum(1 for _, _, v, _ in sorted_entries if v == "false")

    csv_lines = [
        "# test name,category,real vulnerability,CWE",
        "# PHP SAST Benchmark v0.3.1",
        f"# {len(sorted_entries)} test cases ({tp} TP / {tn} TN), 25 CWE categories",
        "# Anti-target-leakage: generic filenames, no annotations",
    ]
    for new_key, cat, vuln, cwe in sorted_entries:
        csv_lines.append(f"{new_key},{cat},{vuln},{cwe}")

    NEW_CSV.write_text("\n".join(csv_lines) + "\n", encoding="utf-8")
    print(f"Wrote {NEW_CSV.name} ({len(sorted_entries)} entries, {tp} TP / {tn} TN)")

    full_map = {
        "seed": SEED,
        "file_count": len(rename_map),
        "file_map": rename_map,
        "key_map": key_map,
    }
    RENAME_MAP_FILE.write_text(json.dumps(full_map, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(f"Wrote {RENAME_MAP_FILE.name}")
    print("Done. Review migration/ then swap.")


if __name__ == "__main__":
    main()
