#!/usr/bin/env python3
"""Rust benchmark anti-target-leakage migration: v0.5.0 -> v0.5.1

Strips all comments from testcode/*.rs files, renames to opaque
benchmark_test_NNNNN.rs, and generates updated CSV.

Run from gorustbash_benchmark root:
    python3 scripts/migrate_rust_v051.py
"""

import json
import os
import random
import re
import shutil
import sys
from pathlib import Path

BENCH_ROOT = Path(__file__).resolve().parent.parent
RUST_DIR = BENCH_ROOT / "rust"
TESTCODE_DIR = RUST_DIR / "testcode"
MIGRATION_DIR = TESTCODE_DIR / "migration"
OLD_CSV = RUST_DIR / "expectedresults-0.5.0.csv"
NEW_CSV = RUST_DIR / "expectedresults-0.5.1.csv"
RENAME_MAP_FILE = RUST_DIR / "rename_map.json"

SHUFFLE_SEED = 20260407


def filename_to_old_csv_key(filename):
    """Convert 'sqli_001.rs' -> 'testcodeSqli001',
    'race_condition_021.rs' -> 'testcodeRaceCondition021'."""
    base = filename.replace(".rs", "")
    parts = base.rsplit("_", 1)
    if len(parts) != 2 or not parts[1].isdigit():
        raise ValueError("Cannot parse filename: %s" % filename)
    category_raw = parts[0]
    number = parts[1]
    category_camel = "".join(w.capitalize() for w in category_raw.split("_"))
    return "testcode%s%s" % (category_camel, number)


def find_comment_start(line):
    """Find index of first // that is a real comment (outside string literals).
    Returns -1 if no comment found."""
    in_string = False
    i = 0
    while i < len(line):
        c = line[i]
        if in_string:
            if c == "\\":
                i += 2
                continue
            if c == '"':
                in_string = False
        else:
            if c == '"':
                in_string = True
            elif c == "/" and i + 1 < len(line) and line[i + 1] == "/":
                return i
        i += 1
    return -1


def strip_comments(lines):
    """Strip all comments from a Rust testcode file."""
    result = []
    for raw_line in lines:
        line = raw_line.rstrip("\n").rstrip("\r")

        # Type 1: //! doc comment
        if re.match(r"^//!", line):
            continue

        # Type 2: standalone vuln-code-snippet start/end marker
        if re.match(r"^\s*//\s*vuln-code-snippet\s+(start|end)", line):
            continue

        # Type 3: trailing vuln-code-snippet target-line
        m = re.search(r"\s*//\s*vuln-code-snippet\s+target-line\s+\S+", line)
        if m:
            code = line[: m.start()].rstrip()
            if code:
                result.append(code + "\n")
            continue

        # Type 4: standalone // comment line
        if re.match(r"^\s*//", line):
            continue

        # Type 5: trailing // comment on a code line
        pos = find_comment_start(line)
        if pos > 0:
            code = line[:pos].rstrip()
            if code:
                result.append(code + "\n")
                continue

        # Keep line as-is
        result.append(line + "\n")

    # Collapse consecutive blank lines into one
    cleaned = []
    prev_blank = False
    for line in result:
        is_blank = line.strip() == ""
        if is_blank and prev_blank:
            continue
        cleaned.append(line)
        prev_blank = is_blank

    # Strip leading/trailing blank lines
    while cleaned and cleaned[0].strip() == "":
        cleaned.pop(0)
    while cleaned and cleaned[-1].strip() == "":
        cleaned.pop()

    if cleaned and not cleaned[-1].endswith("\n"):
        cleaned[-1] += "\n"

    return cleaned


def main():
    print("=" * 64)
    print("  Rust Anti-Target-Leakage Migration v0.5.0 -> v0.5.1")
    print("=" * 64)
    print()

    # ------------------------------------------------------------------
    # Phase 1: Comment Stripping
    # ------------------------------------------------------------------
    print("[Phase 1] Comment Stripping")
    print("  Source: %s" % TESTCODE_DIR)
    print("  Output: %s" % MIGRATION_DIR)

    if MIGRATION_DIR.exists():
        shutil.rmtree(MIGRATION_DIR)
    MIGRATION_DIR.mkdir()

    test_files = sorted(
        f.name
        for f in TESTCODE_DIR.iterdir()
        if f.name.endswith(".rs") and not f.name.startswith("shared")
    )
    print("  Found %d testcode files" % len(test_files))

    total_lines_removed = 0
    for filename in test_files:
        src = TESTCODE_DIR / filename
        with open(src, "r", encoding="utf-8") as f:
            original = f.readlines()

        cleaned = strip_comments(original)

        dst = MIGRATION_DIR / filename
        with open(dst, "w", encoding="utf-8", newline="\n") as f:
            f.writelines(cleaned)

        total_lines_removed += len(original) - len(cleaned)

    print("  Stripped %d files, %d total lines removed" % (len(test_files), total_lines_removed))

    # Validation gate 1
    errors = []
    for filename in test_files:
        content = (MIGRATION_DIR / filename).read_text(encoding="utf-8")
        if "CWE-" in content:
            errors.append("CWE reference in %s" % filename)
        if "vuln-code-snippet" in content:
            errors.append("vuln-code-snippet in %s" % filename)
        if "pub fn handle" not in content:
            errors.append("Missing pub fn handle in %s" % filename)
        for i, line in enumerate(content.splitlines(), 1):
            if re.match(r"^\s*//", line):
                errors.append("Standalone comment at %s:%d: %s" % (filename, i, line.strip()))

    if errors:
        print("  VALIDATION FAILED (%d errors):" % len(errors))
        for e in errors[:30]:
            print("    %s" % e)
        if len(errors) > 30:
            print("    ... and %d more" % (len(errors) - 30))
        sys.exit(1)

    print("  Validation: PASS")
    print()

    # ------------------------------------------------------------------
    # Phase 2: File Renaming
    # ------------------------------------------------------------------
    print("[Phase 2] File Renaming (seed=%d)" % SHUFFLE_SEED)

    shuffled = test_files[:]
    random.seed(SHUFFLE_SEED)
    random.shuffle(shuffled)

    rename_map = {}
    for new_idx, old_filename in enumerate(shuffled, 1):
        new_filename = "benchmark_test_%05d.rs" % new_idx
        old_key = filename_to_old_csv_key(old_filename)
        new_key = "BenchmarkTest%05d" % new_idx

        rename_map[old_filename] = {
            "new_file": new_filename,
            "old_csv_key": old_key,
            "new_csv_key": new_key,
        }

    # Two-pass rename to avoid collisions
    for old_filename in rename_map:
        src = MIGRATION_DIR / old_filename
        src.rename(MIGRATION_DIR / (old_filename + ".tmp"))

    for old_filename, info in rename_map.items():
        tmp = MIGRATION_DIR / (old_filename + ".tmp")
        tmp.rename(MIGRATION_DIR / info["new_file"])

    with open(RENAME_MAP_FILE, "w", encoding="utf-8", newline="\n") as f:
        json.dump(rename_map, f, indent=2, sort_keys=True)

    print("  Renamed %d files" % len(rename_map))
    print("  Rename map: %s" % RENAME_MAP_FILE)

    # Validation gate 2
    renamed = [
        f.name
        for f in MIGRATION_DIR.iterdir()
        if f.name.startswith("benchmark_test_") and f.name.endswith(".rs")
    ]
    if len(renamed) != len(test_files):
        print("  VALIDATION FAILED: expected %d, got %d" % (len(test_files), len(renamed)))
        sys.exit(1)

    print("  Validation: PASS (%d files)" % len(renamed))
    print()

    # ------------------------------------------------------------------
    # Phase 3: CSV Migration
    # ------------------------------------------------------------------
    print("[Phase 3] CSV Migration")

    key_map = {info["old_csv_key"]: info["new_csv_key"] for info in rename_map.values()}

    with open(OLD_CSV, "r", encoding="utf-8") as f:
        old_lines = f.readlines()

    new_lines = []
    testcode_mapped = 0
    apps_kept = 0

    for line in old_lines:
        stripped = line.strip()
        if not stripped or stripped.startswith("#"):
            updated = line
            updated = updated.replace("v0.5.0", "v0.5.1")
            updated = updated.replace("expectedresults-0.5.0", "expectedresults-0.5.1")
            new_lines.append(updated)
            continue

        parts = stripped.split(",")
        old_key = parts[0].strip()

        if old_key in key_map:
            parts[0] = key_map[old_key]
            new_lines.append(",".join(parts) + "\n")
            testcode_mapped += 1
        else:
            new_lines.append(line)
            apps_kept += 1

    with open(NEW_CSV, "w", encoding="utf-8", newline="\n") as f:
        f.writelines(new_lines)

    print("  Testcode entries mapped: %d" % testcode_mapped)
    print("  Apps entries unchanged:  %d" % apps_kept)
    print("  Total:                   %d" % (testcode_mapped + apps_kept))
    print("  Written: %s" % NEW_CSV)

    # Validation gate 3
    benchmark_count = 0
    total_data = 0
    seen_keys = set()
    dupes = []
    with open(NEW_CSV, "r", encoding="utf-8") as f:
        for line in f:
            s = line.strip()
            if not s or s.startswith("#"):
                continue
            total_data += 1
            key = s.split(",")[0].strip()
            if key in seen_keys:
                dupes.append(key)
            seen_keys.add(key)
            if key.startswith("BenchmarkTest"):
                benchmark_count += 1

    if dupes:
        print("  VALIDATION FAILED: duplicate keys: %s" % dupes[:5])
        sys.exit(1)
    if benchmark_count != len(test_files):
        print("  VALIDATION FAILED: expected %d BenchmarkTest keys, got %d" % (len(test_files), benchmark_count))
        sys.exit(1)

    print("  Validation: PASS (%d BenchmarkTest + %d apps = %d total, 0 duplicates)" % (
        benchmark_count, apps_kept, total_data))
    print()

    # ------------------------------------------------------------------
    # Copy shared.rs
    # ------------------------------------------------------------------
    shutil.copy2(TESTCODE_DIR / "shared.rs", MIGRATION_DIR / "shared.rs")
    print("  Copied shared.rs to migration/")
    print()

    # ------------------------------------------------------------------
    # Summary
    # ------------------------------------------------------------------
    print("=" * 64)
    print("  MIGRATION READY FOR REVIEW")
    print("=" * 64)
    print()
    print("  Cleaned + renamed files: %s/" % MIGRATION_DIR)
    print("  New CSV:                 %s" % NEW_CSV)
    print("  Rename map:              %s" % RENAME_MAP_FILE)
    print()
    print("  Next steps (manual):")
    print("    1. Spot-check files in migration/")
    print("    2. mkdir testcode/old && mv testcode/*.rs testcode/old/")
    print("    3. mv migration/*.rs testcode/")
    print("    4. Update validate_rust.py, SCORING.md, rust_benchmark.md, CHANGELOG.md")
    print("    5. Run validate_rust.py")
    print()


if __name__ == "__main__":
    main()
