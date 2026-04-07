#!/usr/bin/env python3
"""Ruby testcode anti-leakage migration.

Strips all comments, renames files via seeded shuffle, generates new CSV.
Writes cleaned files to testcode/migration/ for validation before swap.
"""

import json
import os
import random
import re
import sys
from pathlib import Path

BASE = Path(__file__).resolve().parent.parent
TESTCODE = BASE / "ruby" / "testcode"
MIGRATION = TESTCODE / "migration"
CSV_OLD = BASE / "ruby" / "expectedresults-0.3.0.csv"
CSV_NEW = BASE / "ruby" / "expectedresults-0.3.1.csv"
RENAME_MAP = TESTCODE / "rename_map.json"
SEED = 20260407

LEAKY_DEF = re.compile(r"^(\s*def\s+)(xss_|ssti_|dynmethod_|mass_assign_)\w+(.*)")
SNIPPET_TRAIL = re.compile(r"\s+#\s*vuln-code-snippet\s+\S+\s+\S+.*$")


def is_magic(line, idx):
    s = line.lstrip()
    if idx == 0 and s.startswith("#!"):
        return True
    if s.startswith("# frozen_string_literal:"):
        return True
    if s.startswith("# encoding:"):
        return True
    return False


def strip_trailing_comment(line):
    if "#" not in line:
        return line

    in_sq = False
    in_dq = False
    interp = 0
    i = 0
    n = len(line)

    while i < n:
        c = line[i]

        if in_sq:
            if c == "\\" and i + 1 < n:
                i += 2
                continue
            if c == "'":
                in_sq = False
        elif in_dq and interp == 0:
            if c == "\\" and i + 1 < n:
                i += 2
                continue
            if c == "#" and i + 1 < n and line[i + 1] == "{":
                interp = 1
                i += 2
                continue
            if c == '"':
                in_dq = False
        elif interp > 0:
            if c == "{":
                interp += 1
            elif c == "}":
                interp -= 1
        else:
            if c == "'":
                in_sq = True
            elif c == '"':
                in_dq = True
            elif c == "#":
                result = line[:i].rstrip()
                return result + ("\n" if line.endswith("\n") else "")

        i += 1

    return line


def clean_file(lines):
    result = []
    in_heredoc = False
    heredoc_delim = None

    for idx, line in enumerate(lines):
        if in_heredoc:
            result.append(line)
            if line.strip() == heredoc_delim:
                in_heredoc = False
            continue

        stripped = line.lstrip()

        if stripped.startswith("#"):
            if is_magic(stripped, idx):
                result.append(line)
            continue

        line = strip_trailing_comment(line)

        m = LEAKY_DEF.match(line)
        if m:
            replacement = m.group(1) + "handler" + m.group(3)
            if line.endswith("\n") and not replacement.endswith("\n"):
                replacement += "\n"
            line = replacement

        hm = re.search(r"<<[~-]?['\"]?(\w+)['\"]?", line)
        if hm:
            in_heredoc = True
            heredoc_delim = hm.group(1)

        result.append(line)

    cleaned = []
    prev_blank = False
    for line in result:
        is_blank = line.strip() == ""
        if is_blank and prev_blank:
            continue
        cleaned.append(line)
        prev_blank = is_blank

    while cleaned and cleaned[-1].strip() == "":
        cleaned.pop()
    if cleaned and not cleaned[-1].endswith("\n"):
        cleaned[-1] += "\n"

    return cleaned


def extract_key(filepath):
    with open(filepath, "r", encoding="utf-8") as f:
        for line in f:
            m = re.search(r"vuln-code-snippet\s+start\s+(\S+)", line)
            if m:
                return m.group(1)
    return None


def main():
    test_files = sorted(
        f for f in TESTCODE.glob("*.rb") if f.name != "shared.rb"
    )
    print("Files found: %d" % len(test_files))

    if len(test_files) != 1288:
        print("ERROR: Expected 1288 files, got %d" % len(test_files), file=sys.stderr)
        sys.exit(1)

    MIGRATION.mkdir(exist_ok=True)

    file_keys = {}
    missing_keys = []
    for f in test_files:
        key = extract_key(f)
        if key:
            file_keys[f.name] = key
        else:
            missing_keys.append(f.name)

    if missing_keys:
        print("ERROR: No annotation key in: %s" % ", ".join(missing_keys), file=sys.stderr)
        sys.exit(1)

    random.seed(SEED)
    shuffled = list(test_files)
    random.shuffle(shuffled)

    rename_map = {}
    for idx, old_file in enumerate(shuffled):
        num = idx + 1
        rename_map[old_file.name] = {
            "new_filename": "benchmark_test_%05d.rb" % num,
            "new_key": "BenchmarkTest%05d" % num,
            "old_key": file_keys[old_file.name],
        }

    stats = {"files": 0, "comments_stripped": 0, "funcs_renamed": 0}

    for old_file in test_files:
        info = rename_map[old_file.name]

        with open(old_file, "r", encoding="utf-8") as f:
            original = f.readlines()

        orig_comments = sum(1 for l in original if l.lstrip().startswith("#"))

        cleaned = clean_file(original)

        new_comments = sum(1 for l in cleaned if l.lstrip().startswith("#"))
        stats["comments_stripped"] += orig_comments - new_comments

        for ol in original:
            if LEAKY_DEF.match(ol):
                stats["funcs_renamed"] += 1
                break

        out_path = MIGRATION / info["new_filename"]
        with open(out_path, "w", encoding="utf-8", newline="\n") as f:
            f.writelines(cleaned)

        stats["files"] += 1

    with open(RENAME_MAP, "w", encoding="utf-8") as f:
        json.dump(rename_map, f, indent=2)

    key_map = {v["old_key"]: v["new_key"] for v in rename_map.values()}

    csv_lines = [
        "# test name,category,real vulnerability,CWE\n",
        "# Ruby SAST Benchmark v0.3.1\n",
    ]
    tp = tn = skipped = 0

    with open(CSV_OLD, "r", encoding="utf-8") as f:
        for line in f:
            if line.startswith("#") or not line.strip():
                continue
            parts = line.strip().split(",")
            if len(parts) < 4:
                continue
            old_key = parts[0]
            if old_key not in key_map:
                skipped += 1
                continue
            new_key = key_map[old_key]
            csv_lines.append("%s,%s,%s,%s\n" % (new_key, parts[1], parts[2], parts[3]))
            if parts[2] == "true":
                tp += 1
            else:
                tn += 1

    total = tp + tn
    csv_lines.insert(2, "# %d test cases (%d TP / %d TN), 27 CWE categories\n" % (total, tp, tn))

    with open(CSV_NEW, "w", encoding="utf-8", newline="\n") as f:
        f.writelines(csv_lines)

    print()
    print("Migration complete:")
    print("  Files processed:     %d" % stats["files"])
    print("  Comments stripped:   %d" % stats["comments_stripped"])
    print("  Functions renamed:   %d" % stats["funcs_renamed"])
    print("  CSV entries:         %d (%d TP / %d TN)" % (total, tp, tn))
    print("  Apps entries skipped: %d" % skipped)
    print("  Migration dir:       %s" % MIGRATION)
    print("  Rename map:          %s" % RENAME_MAP)
    print("  New CSV:             %s" % CSV_NEW)


if __name__ == "__main__":
    main()
