#!/usr/bin/env python3
"""
Bash Benchmark Path B Migration: Multi-test files -> 1-file-1-test
====================================================================
Each vuln-code-snippet start/end block becomes one standalone
benchmark_test_NNNNN.sh file.  All comments stripped. All annotation
markers stripped. Preamble variables injected where referenced.

Output:
  bash/testcode/migration/benchmark_test_NNNNN.sh  (1,058 files)
  bash/expectedresults-0.5.1.csv

Run:
  python3 scripts/migrate_bash_to_flat.py [--dry-run]
"""

import argparse
import os
import re
import sys
from collections import Counter, defaultdict
from pathlib import Path

BENCH_ROOT = Path(__file__).resolve().parent.parent / "bash"
CSV_FILE = BENCH_ROOT / "expectedresults-0.5.0.csv"
MIGRATION_DIR = BENCH_ROOT / "testcode" / "migration"
NEW_CSV = BENCH_ROOT / "expectedresults-0.5.1.csv"

SCAN_DIRS = [BENCH_ROOT / "testcode", BENCH_ROOT / "apps"]

PAT_START = re.compile(r"vuln-code-snippet\s+start\s+(\S+)")
PAT_END = re.compile(r"vuln-code-snippet\s+end\s+(\S+)")
# A "simple" preamble var: VAR="literal" or VAR=/path — no command substitution
PAT_SIMPLE_VAR = re.compile(r'^([A-Z_][A-Z0-9_]*)="[^"$`]*"$|^([A-Z_][A-Z0-9_]*)=\'[^\']*\'$|^([A-Z_][A-Z0-9_]*)=/\S+$')


# ---------------------------------------------------------------------------
# CSV
# ---------------------------------------------------------------------------

def parse_csv(path):
    entries = {}
    with open(path, encoding="utf-8") as f:
        for line in f:
            s = line.strip()
            if not s or s.startswith("#"):
                continue
            parts = s.split(",")
            if len(parts) < 4:
                continue
            key = parts[0].strip()
            entries[key] = {
                "category": parts[1].strip(),
                "vulnerable": parts[2].strip().lower() == "true",
                "cwe": parts[3].strip(),
            }
    return entries


# ---------------------------------------------------------------------------
# Comment stripping (bash-aware)
# ---------------------------------------------------------------------------

def strip_inline_comment(line):
    """
    Remove trailing # comment from a code line, respecting:
    - Single-quoted strings  (no substitution inside)
    - Double-quoted strings
    - Backslash escapes
    - $# special variable (argument count) — NOT a comment
    - ${#var} string-length operator — NOT a comment
    - $'...' ANSI-C quoting — the # inside is not a comment
    Returns the line with inline comment removed and trailing whitespace stripped.
    """
    result = []
    in_single = False
    in_double = False
    in_brace = 0   # depth inside ${...}
    i = 0
    while i < len(line):
        c = line[i]

        # Backslash escape (not inside single quotes)
        if c == "\\" and i + 1 < len(line) and not in_single:
            result.append(c)
            result.append(line[i + 1])
            i += 2
            continue

        if c == "'" and not in_double:
            # $'...' ANSI-C quoting: toggle single but don't break on interior #
            in_single = not in_single
        elif c == '"' and not in_single:
            in_double = not in_double
        elif c == '{' and not in_single and not in_double and result and result[-1] == '$':
            # Entering ${...} — track depth to handle ${#var}
            in_brace += 1
        elif c == '{' and not in_single and not in_double and in_brace > 0:
            in_brace += 1
        elif c == '}' and not in_single and not in_double and in_brace > 0:
            in_brace -= 1
        elif c == '#' and not in_single and not in_double:
            # NOT a comment if:
            #   - Preceded by $ (i.e., $# — argument count)
            #   - Inside ${...} at any depth (i.e., ${#var} — string length)
            if in_brace > 0:
                pass  # inside ${...}, keep it
            elif result and result[-1] == '$':
                pass  # $# special variable, keep it
            else:
                break  # real comment — strip from here
        result.append(c)
        i += 1
    return "".join(result).rstrip()


def strip_comments(lines):
    """
    Return lines with all comments removed.
    Rules:
      - #! shebang lines: skipped (caller adds its own shebang)
      - Standalone comment lines (possibly indented): removed
      - Inline comments: stripped from code lines
      - Heredoc body: preserved verbatim (# inside heredoc is data)
      - vuln-code-snippet annotation lines: removed entirely
    Blank lines are preserved (they aid readability in extracted functions).
    """
    out = []
    in_heredoc = False
    heredoc_delim = None

    for raw in lines:
        line = raw.rstrip("\n")

        # --- heredoc body: pass through unchanged ---
        if in_heredoc:
            # Check for closing delimiter (bare word, possibly indented for <<-)
            stripped_line = line.strip()
            if stripped_line == heredoc_delim:
                in_heredoc = False
                heredoc_delim = None
            out.append(line)
            continue

        # --- detect heredoc opening on this line ---
        hm = re.search(r"<<-?\s*['\"]?(\w+)['\"]?", line)
        if hm and "<<" in line:
            potential_delim = hm.group(1)
            # Only enter heredoc mode if this isn't inside a comment
            # (quick check: if '#' appears before '<<' it's a comment line)
            lt = line.lstrip()
            if not lt.startswith("#"):
                in_heredoc = True
                heredoc_delim = potential_delim
                # Strip inline annotation if present; keep the heredoc-opening code
                clean = strip_inline_comment(line)
                if clean.strip():
                    out.append(clean)
                continue

        # --- shebang: skip (we write our own) ---
        if line.lstrip().startswith("#!"):
            continue

        # --- annotation marker lines: skip entirely ---
        if "vuln-code-snippet" in line:
            # Could be standalone comment or inline annotation on a code line
            stripped = line.lstrip()
            if stripped.startswith("#"):
                continue  # pure comment line with annotation — drop it
            else:
                # inline annotation on a code line — strip just the annotation part
                clean = re.sub(r"\s*#\s*vuln-code-snippet\S*.*$", "", line).rstrip()
                if clean.strip():
                    out.append(clean)
                continue

        # --- standalone comment lines: skip ---
        if re.match(r"^\s*#", line):
            continue

        # --- code line: strip any inline comment ---
        if "#" in line:
            clean = strip_inline_comment(line)
            out.append(clean)
        else:
            out.append(line)

    return out


# ---------------------------------------------------------------------------
# Preamble extraction
# ---------------------------------------------------------------------------

def extract_simple_preamble(all_lines):
    """
    Collect simple variable assignments that appear before the first function
    definition or first annotation marker.  Skip complex ones (command sub etc).
    Returns list of assignment strings.
    """
    preamble = []
    for raw in all_lines:
        line = raw.rstrip()
        # Stop at first function definition
        if re.match(r"^\w+\s*\(\)\s*\{?\s*$", line) or re.match(r"^function\s+\w+", line):
            break
        # Stop at first annotation
        if "vuln-code-snippet" in line:
            break
        # Must be a simple top-level variable assignment
        if PAT_SIMPLE_VAR.match(line):
            preamble.append(line)
    return preamble


def preamble_vars_used(preamble, block_lines):
    """
    Return only the preamble entries whose variable name appears in block_lines.
    """
    used = []
    block_text = "".join(block_lines)
    for entry in preamble:
        m = re.match(r"^([A-Z_][A-Z0-9_]*)", entry)
        if m:
            var_name = m.group(1)
            if ("$" + var_name) in block_text or ("${" + var_name) in block_text:
                used.append(entry)
    return used


# ---------------------------------------------------------------------------
# Test case extraction
# ---------------------------------------------------------------------------

def extract_test_cases(filepath):
    """
    Parse a single .sh file and return:
      cases: list of {key, block_lines, preamble}
      preamble: simple var assignments from file top
    """
    with open(filepath, encoding="utf-8", errors="replace") as f:
        all_lines = f.readlines()

    preamble = extract_simple_preamble(all_lines)

    cases = []
    open_starts = {}  # key -> line_index

    for i, raw in enumerate(all_lines):
        line = raw.rstrip()
        m = PAT_START.search(line)
        if m:
            key = m.group(1)
            open_starts[key] = i

        m = PAT_END.search(line)
        if m:
            key = m.group(1)
            if key in open_starts:
                start_idx = open_starts.pop(key)
                # Lines between start and end markers (exclusive)
                block_lines = all_lines[start_idx + 1 : i]
                cases.append({"key": key, "block_lines": block_lines})

    # Attach preamble to each case
    for case in cases:
        case["preamble"] = preamble_vars_used(preamble, case["block_lines"])

    return cases


# ---------------------------------------------------------------------------
# File writer
# ---------------------------------------------------------------------------

def is_complete_function(stripped_lines):
    """
    Returns True if the stripped block starts with a function definition
    (i.e., is a self-contained syntactic unit that bash -n will accept).
    """
    for line in stripped_lines:
        if line.strip():
            # Function definition: name() { or name() \n {  or function name
            if re.match(r"^\w+\s*\(\)", line) or re.match(r"^function\s+\w+", line):
                return True
            return False
    return False


def wrap_in_function(stripped_lines):
    """
    Wrap lines in a synthetic test_case() { ... } function.
    Used when the extracted block is not itself a complete function.
    Indents the body by 4 spaces.
    """
    result = ["test_case() {"]
    for line in stripped_lines:
        if line.strip():
            result.append("    " + line)
        else:
            result.append("")
    result.append("}")
    return result


def write_benchmark_file(outpath, preamble, block_lines):
    """Write one benchmark_test_NNNNN.sh file.
    If the extracted block is not a complete function, wrap it in test_case().
    """
    stripped = strip_comments(block_lines)

    # Remove leading/trailing blank lines from the stripped block
    while stripped and not stripped[0].strip():
        stripped.pop(0)
    while stripped and not stripped[-1].strip():
        stripped.pop()

    # Wrap in synthetic function if needed (handles mid-function annotations
    # and script-level annotation blocks from app files)
    if stripped and not is_complete_function(stripped):
        stripped = wrap_in_function(stripped)

    content_lines = ["#!/bin/bash", ""]
    if preamble:
        content_lines.extend(preamble)
        content_lines.append("")
    content_lines.extend(stripped)
    content_lines.append("")  # trailing newline

    outpath.write_text("\n".join(content_lines), encoding="utf-8")


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    parser = argparse.ArgumentParser(description="Migrate bash benchmark to 1-file-1-test")
    parser.add_argument("--dry-run", action="store_true", help="Print plan but write nothing")
    args = parser.parse_args()

    # Load CSV ground truth
    csv_entries = parse_csv(CSV_FILE)
    print(f"Loaded {len(csv_entries)} CSV entries from {CSV_FILE.name}")

    # Collect all test cases from all annotated files
    all_cases = []
    for scan_dir in SCAN_DIRS:
        if not scan_dir.is_dir():
            continue
        for root, dirs, files in os.walk(scan_dir):
            dirs[:] = [d for d in dirs if d not in (
                ".git", "node_modules", ".auditor_venv", ".pf", "migration"
            )]
            for fn in sorted(files):
                if not fn.endswith(".sh"):
                    continue
                filepath = Path(root) / fn
                cases = extract_test_cases(filepath)
                for case in cases:
                    case["source_file"] = str(filepath.relative_to(BENCH_ROOT))
                all_cases.extend(cases)

    print(f"Found {len(all_cases)} annotated test cases in source files")

    # Warn about cases not in CSV
    missing_from_csv = [c for c in all_cases if c["key"] not in csv_entries]
    if missing_from_csv:
        print(f"\nWARNING: {len(missing_from_csv)} source annotations have no CSV entry:")
        for c in missing_from_csv[:5]:
            print(f"  {c['key']}  (from {c['source_file']})")
        if len(missing_from_csv) > 5:
            print(f"  ... and {len(missing_from_csv) - 5} more")

    # Warn about CSV entries with no annotation
    ann_keys = {c["key"] for c in all_cases}
    missing_annotation = [k for k in csv_entries if k not in ann_keys]
    if missing_annotation:
        print(f"\nWARNING: {len(missing_annotation)} CSV entries have no source annotation:")
        for k in missing_annotation[:5]:
            print(f"  {k}")

    # Build the ordered list: only cases that exist in CSV
    ordered = [c for c in all_cases if c["key"] in csv_entries]
    print(f"\n{len(ordered)} test cases will be migrated")

    if not args.dry_run:
        MIGRATION_DIR.mkdir(parents=True, exist_ok=True)

    # Generate files and new CSV rows
    new_csv_rows = []
    file_count = 0
    issues = []

    for i, case in enumerate(ordered, 1):
        key = case["key"]
        entry = csv_entries[key]
        new_name = f"benchmark_test_{i:05d}"
        new_filename = f"{new_name}.sh"

        vuln_str = "true" if entry["vulnerable"] else "false"
        new_csv_rows.append(f"{new_name},{entry['category']},{vuln_str},{entry['cwe']}")

        if not args.dry_run:
            outpath = MIGRATION_DIR / new_filename
            try:
                write_benchmark_file(outpath, case["preamble"], case["block_lines"])
                file_count += 1
            except Exception as e:
                issues.append(f"{key} -> {new_filename}: {e}")

    # Write new CSV
    if not args.dry_run:
        with open(NEW_CSV, "w", encoding="utf-8", newline="\n") as f:
            f.write("# test name,category,real vulnerability,CWE,Benchmark version: 0.5.1,2026-04-07\n")
            for row in new_csv_rows:
                f.write(row + "\n")
        print(f"Written {len(new_csv_rows)} rows to {NEW_CSV.name}")
        print(f"Written {file_count} files to {MIGRATION_DIR.relative_to(BENCH_ROOT)}/")
    else:
        print(f"\n[DRY RUN] Would write {len(new_csv_rows)} files and new CSV")

    # Balance check
    tp = sum(1 for r in new_csv_rows if ",true," in r)
    tn = sum(1 for r in new_csv_rows if ",false," in r)
    total = tp + tn
    print(f"\nBalance: {tp} TP / {tn} TN / {total} total")
    if tp == tn:
        print("50/50 balance: EXACT")
    else:
        print(f"50/50 balance: OFF by {abs(tp - tn)}")

    # Per-category check
    cat_counts = defaultdict(lambda: {"tp": 0, "tn": 0})
    for row in new_csv_rows:
        parts = row.split(",")
        cat = parts[1]
        vuln = parts[2] == "true"
        if vuln:
            cat_counts[cat]["tp"] += 1
        else:
            cat_counts[cat]["tn"] += 1

    print(f"\n{'Category':<25} {'TP':>4} {'TN':>4} {'Total':>6} {'Min':>5}")
    print("-" * 48)
    for cat in sorted(cat_counts):
        c = cat_counts[cat]
        t, s = c["tp"], c["tn"]
        flag = " <-- BELOW 25" if t < 25 or s < 25 else ""
        print(f"{cat:<25} {t:>4} {s:>4} {t+s:>6}{flag}")

    if issues:
        print(f"\nISSUES ({len(issues)}):")
        for iss in issues:
            print(f"  {iss}")

    print("\nDone." if not args.dry_run else "\nDry run complete.")


if __name__ == "__main__":
    main()
