#!/usr/bin/env python3
"""Bash benchmark migration: split multi-test .sh files into 1-file-1-test.

Reads from testcode/archived_v0.5.0/, extracts each vuln-code-snippet block,
strips all comments, writes clean files to testcode/migration/ with shuffled
generic names, and generates expectedresults-0.5.1.csv.

Apps/ entries in the CSV are preserved unchanged.
"""

import os
import re
import random
import csv
from pathlib import Path

SEED = 20260407
BASH_DIR = Path(__file__).resolve().parent.parent / "bash"
ARCHIVE_DIR = BASH_DIR / "testcode" / "archived_v0.5.0"
MIGRATION_DIR = BASH_DIR / "testcode" / "migration"
CSV_IN = BASH_DIR / "expectedresults-0.5.0.csv"
CSV_OUT = BASH_DIR / "expectedresults-0.5.1.csv"


def extract_blocks(filepath):
    """Extract all vuln-code-snippet blocks from a .sh file.

    Returns list of (snippet_id, [lines_of_code]).
    """
    blocks = []
    current_id = None
    current_lines = []

    with open(filepath, "r", encoding="utf-8") as f:
        for line in f:
            stripped = line.rstrip("\n")

            start_m = re.match(r'^\s*#\s*vuln-code-snippet\s+start\s+(\S+)', stripped)
            if start_m:
                current_id = start_m.group(1)
                current_lines = []
                continue

            end_m = re.match(r'^\s*#\s*vuln-code-snippet\s+end\s+(\S+)', stripped)
            if end_m:
                if current_id:
                    blocks.append((current_id, current_lines))
                current_id = None
                current_lines = []
                continue

            if current_id is not None:
                current_lines.append(stripped)

    return blocks


def strip_comments(lines):
    """Remove comments from bash code lines, heredoc-aware.

    - Deletes standalone comment lines (line is only a comment)
    - Strips trailing '# vuln-code-snippet ...' annotations
    - Preserves # inside heredocs, strings, and ${#...} patterns
    """
    cleaned = []
    in_heredoc = False
    heredoc_delim = None

    for line in lines:
        # Track heredoc state
        if in_heredoc:
            if line.strip() == heredoc_delim:
                in_heredoc = False
            cleaned.append(line)
            continue

        # Check for heredoc start
        heredoc_m = re.search(r'<<-?\s*[\'"]?(\w+)[\'"]?', line)
        if heredoc_m:
            in_heredoc = True
            heredoc_delim = heredoc_m.group(1)

        # Strip trailing vuln-code-snippet annotation
        line = re.sub(r'\s*#\s*vuln-code-snippet\s+\S+\s+\S+\s*$', '', line)

        # Check if line is a standalone comment (not shebang)
        stripped = line.strip()
        if stripped.startswith('#') and not stripped.startswith('#!'):
            continue

        # Skip empty lines that resulted from stripping
        if not stripped:
            continue

        cleaned.append(line)

    return cleaned


def build_file_content(snippet_lines):
    """Wrap cleaned code in a proper standalone bash file."""
    content = ["#!/bin/bash"]
    content.extend(snippet_lines)
    return "\n".join(content) + "\n"


def main():
    MIGRATION_DIR.mkdir(parents=True, exist_ok=True)

    # Phase 1: Extract all blocks from all archived files
    all_blocks = {}  # snippet_id -> cleaned lines
    for sh_file in sorted(ARCHIVE_DIR.glob("*.sh")):
        blocks = extract_blocks(sh_file)
        for snippet_id, lines in blocks:
            cleaned = strip_comments(lines)
            if snippet_id in all_blocks:
                print(f"WARNING: duplicate snippet ID '{snippet_id}' in {sh_file.name}")
            all_blocks[snippet_id] = cleaned

    print(f"Extracted {len(all_blocks)} test cases from {len(list(ARCHIVE_DIR.glob('*.sh')))} files")

    # Phase 2: Read CSV, separate testcode vs apps entries
    csv_header = None
    testcode_rows = []  # rows whose test name matches a snippet ID
    apps_rows = []      # rows whose test name is in apps/ (not in snippets)

    with open(CSV_IN, "r", encoding="utf-8") as f:
        for line in f:
            if line.startswith("#"):
                csv_header = line  # keep last header line
                continue
            parts = line.strip().split(",")
            if len(parts) < 4:
                continue
            test_name = parts[0].strip()
            if test_name in all_blocks:
                testcode_rows.append(parts)
            else:
                apps_rows.append(parts)

    print(f"CSV testcode entries: {len(testcode_rows)}")
    print(f"CSV apps entries: {len(apps_rows)}")

    # Sanity: every snippet should be in CSV
    csv_names = {r[0].strip() for r in testcode_rows}
    missing = set(all_blocks.keys()) - csv_names
    if missing:
        print(f"ERROR: {len(missing)} snippets not in CSV: {sorted(missing)[:10]}")
        return

    # Phase 3: Shuffle and assign numbers
    snippet_ids = sorted(all_blocks.keys())
    random.seed(SEED)
    random.shuffle(snippet_ids)

    id_to_num = {}
    for i, sid in enumerate(snippet_ids, start=1):
        id_to_num[sid] = f"{i:05d}"

    # Phase 4: Write clean files to migration/
    for sid, num in id_to_num.items():
        content = build_file_content(all_blocks[sid])
        outpath = MIGRATION_DIR / f"benchmark_test_{num}.sh"
        with open(outpath, "w", encoding="utf-8", newline="\n") as f:
            f.write(content)

    print(f"Wrote {len(id_to_num)} files to {MIGRATION_DIR}")

    # Phase 5: Generate new CSV
    with open(CSV_OUT, "w", encoding="utf-8", newline="\n") as f:
        f.write(f"# test name,category,real vulnerability,CWE,Benchmark version: 0.5.1,2026-04-08\n")

        # Write testcode entries with new names
        for parts in testcode_rows:
            old_name = parts[0].strip()
            num = id_to_num[old_name]
            new_name = f"BenchmarkTest{num}"
            f.write(f"{new_name},{parts[1].strip()},{parts[2].strip()},{parts[3].strip()}\n")

        # Write apps entries unchanged
        for parts in apps_rows:
            f.write(",".join(p.strip() for p in parts) + "\n")

    print(f"Wrote CSV: {CSV_OUT}")
    print(f"  Testcode entries: {len(testcode_rows)} (renamed)")
    print(f"  Apps entries: {len(apps_rows)} (unchanged)")

    # Phase 6: Quick validation
    written_files = list(MIGRATION_DIR.glob("benchmark_test_*.sh"))
    print(f"\nValidation:")
    print(f"  Files written: {len(written_files)}")
    print(f"  Expected: {len(all_blocks)}")

    # Check for vuln-code-snippet leakage
    leaks = 0
    for f in written_files:
        text = f.read_text(encoding="utf-8")
        if "vuln-code-snippet" in text:
            leaks += 1
            print(f"  LEAK: {f.name}")
    print(f"  vuln-code-snippet leaks: {leaks}")

    # Check for CWE leakage
    cwe_leaks = 0
    for f in written_files:
        text = f.read_text(encoding="utf-8")
        if re.search(r'#.*CWE-\d+', text):
            cwe_leaks += 1
    print(f"  CWE comment leaks: {cwe_leaks}")

    # Check bash syntax on a sample
    print(f"\n  Run 'bash -n testcode/migration/benchmark_test_00001.sh' to verify syntax")


if __name__ == "__main__":
    main()
