#!/usr/bin/env python3
"""PHP SAST Benchmark Fidelity Validator v2.0

Filename-based validation (post-leakage migration). No annotations.
  L1 -- Structural integrity (CSV <-> file cross-reference)
  L2 -- Naming convention (benchmark_test_NNNNN.php, benchmarkTestNNNNN function)
  L3 -- Schema validation (required fields, valid CWEs, valid categories)
  L4 -- Anti-target-leakage (no annotations, no CWE mentions, no category in comments)
  L5 -- Scoring pipeline readiness (filename-based scoring support)

Exit 0 if all checks pass, 1 if any ERRORS, 2 if only WARNINGS.
No dependencies -- stdlib only.
"""

import hashlib
import json
import re
import sys
from collections import defaultdict
from pathlib import Path

# ============================================================================
# Configuration
# ============================================================================
SCRIPT_DIR = Path(__file__).resolve().parent
BENCH_ROOT = SCRIPT_DIR.parent
PHP_DIR = BENCH_ROOT / "php"
CSV_FILE = PHP_DIR / "expectedresults-0.3.2.csv"
CONVERTER_PY = SCRIPT_DIR / "convert_theauditor.py"
TESTCODE_DIR = PHP_DIR / "testcode"

PAT_BENCHMARK_FILE = re.compile(r"^benchmark_test_(\d{5})\.php$")
PAT_BENCHMARK_KEY = re.compile(r"^BenchmarkTest(\d{5})$")
PAT_ENTRY_FUNC = re.compile(r"^function\s+benchmarkTest(\d{5})\(", re.MULTILINE)
PAT_ANNOTATION = re.compile(r"vuln-code-snippet")
PAT_CWE_MENTION = re.compile(r"CWE-\d+", re.IGNORECASE)
PAT_STANDALONE_COMMENT = re.compile(r"^\s*//\s")

VALID_CWES = {
    20, 22, 78, 79, 89, 90, 94, 98, 113, 327, 328, 330,
    352, 434, 470, 502, 601, 611, 614, 621, 627, 697, 798,
    915, 918, 1336,
}

VALID_CATEGORIES = {
    "sqli", "xss", "cmdi", "fileinclusion", "pathtraver", "deserial",
    "ssrf", "codeinj", "typejuggling", "extract", "variablevars",
    "unsafereflect", "xxe", "fileupload", "redirect", "weakhash",
    "weakrand", "weakcipher", "hardcodedcreds", "csrf", "headerinj",
    "ldapi", "securecookie", "massassign", "ssti",
}

errors = []
warnings = []


# ============================================================================
# L1: Structural Integrity
# ============================================================================
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
                errors.append("L3 Invalid CWE '%s' for key '%s' at CSV line %d" % (cwe_str, key, line_num))
                cwe = -1

            entries[key] = {
                "key": key,
                "category": category,
                "vulnerable": vulnerable_str == "true",
                "cwe": cwe,
            }

    return entries


def scan_test_files():
    files = {}
    if not TESTCODE_DIR.is_dir():
        errors.append("L1 testcode directory not found: %s" % TESTCODE_DIR)
        return files

    for f in sorted(TESTCODE_DIR.glob("*.php")):
        if f.name == "shared.php":
            continue
        m = PAT_BENCHMARK_FILE.match(f.name)
        if m:
            num = m.group(1)
            csv_key = "BenchmarkTest" + num
            files[csv_key] = f
        else:
            errors.append("L2 Non-standard filename in testcode/: %s" % f.name)

    return files


def check_structural(csv_entries, test_files):
    csv_keys = set(csv_entries.keys())
    file_keys = set(test_files.keys())

    for key in sorted(csv_keys - file_keys):
        errors.append("L1 Orphan CSV: '%s' has no matching file" % key)
    for key in sorted(file_keys - csv_keys):
        errors.append("L1 Orphan file: '%s' has no CSV entry" % key)


# ============================================================================
# L2: Naming Convention
# ============================================================================
def check_naming(test_files):
    for csv_key, filepath in test_files.items():
        m = PAT_BENCHMARK_KEY.match(csv_key)
        if not m:
            errors.append("L2 CSV key '%s' does not match BenchmarkTestNNNNN pattern" % csv_key)
            continue

        expected_num = m.group(1)
        content = filepath.read_text(encoding="utf-8", errors="replace")

        func_match = PAT_ENTRY_FUNC.search(content)
        if not func_match:
            errors.append("L2 No benchmarkTestNNNNN entry function in %s" % filepath.name)
        elif func_match.group(1) != expected_num:
            errors.append(
                "L2 Function number mismatch in %s: function has %s, file has %s"
                % (filepath.name, func_match.group(1), expected_num)
            )


# ============================================================================
# L3: Schema Validation
# ============================================================================
def check_schema(csv_entries):
    all_categories = set()

    for key, info in csv_entries.items():
        cwe = info.get("cwe")
        if cwe is not None and cwe not in VALID_CWES:
            warnings.append("L3 CWE %d for key '%s' not in known set" % (cwe, key))

        cat = info.get("category")
        if cat:
            all_categories.add(cat)
            if cat not in VALID_CATEGORIES:
                warnings.append("L3 Category '%s' for key '%s' not in expected set" % (cat, key))

    return all_categories


# ============================================================================
# L4: Anti-Target-Leakage
# ============================================================================
def check_leakage(test_files):
    leakage_count = 0

    for csv_key, filepath in test_files.items():
        content = filepath.read_text(encoding="utf-8", errors="replace")

        if PAT_ANNOTATION.search(content):
            errors.append("L4 Annotation remnant in %s" % filepath.name)
            leakage_count += 1

        if PAT_CWE_MENTION.search(content):
            errors.append("L4 CWE mention in %s" % filepath.name)
            leakage_count += 1

        for i, line in enumerate(content.split("\n"), 1):
            if PAT_STANDALONE_COMMENT.match(line):
                errors.append("L4 Standalone comment at %s:%d" % (filepath.name, i))
                leakage_count += 1
                break

    return leakage_count


# ============================================================================
# L5: Scoring Pipeline Readiness
# ============================================================================
def check_scoring_pipeline(csv_entries):
    if not CONVERTER_PY.exists():
        warnings.append("L5 convert_theauditor.py not found")
        return

    converter_content = CONVERTER_PY.read_text(encoding="utf-8")

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
            warnings.append("L5 CWE %d has no VULN_TYPE_TO_CWE mapping" % cwe)

    if '".php"' not in converter_content:
        warnings.append("L5 '.php' extension not in converter extension map")


# ============================================================================
# Report
# ============================================================================
def print_report(csv_entries):
    categories = defaultdict(lambda: {"tp": 0, "tn": 0})
    for info in csv_entries.values():
        cat = info.get("category", "unknown")
        if info.get("vulnerable", False):
            categories[cat]["tp"] += 1
        else:
            categories[cat]["tn"] += 1

    total_tp = sum(c["tp"] for c in categories.values())
    total_tn = sum(c["tn"] for c in categories.values())
    total = total_tp + total_tn

    print("CSV entries:    %d" % len(csv_entries))
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
    print("  PHP SAST Benchmark Fidelity Validator v2.0")
    print("  Filename-based (post-leakage migration)")
    print("=" * 64)
    print()

    print("[L1] Structural Integrity (CSV <-> file cross-reference)")
    csv_entries = parse_csv()
    test_files = scan_test_files()
    check_structural(csv_entries, test_files)
    l1_errors = len(errors)
    print("  CSV keys: %d  |  Test files: %d" % (len(csv_entries), len(test_files)))
    print("  Result: %d errors" % l1_errors)
    print()

    print("[L2] Naming Convention (file + function pattern)")
    check_naming(test_files)
    l2_errors = len(errors) - l1_errors
    print("  Result: %d errors" % l2_errors)
    print()

    print("[L3] Schema Validation (CWEs, categories)")
    all_categories = check_schema(csv_entries)
    l3_errors = len(errors) - l1_errors - l2_errors
    print("  Categories: %d (%s)" % (len(all_categories), ", ".join(sorted(all_categories))))
    print("  Result: %d errors" % l3_errors)
    print()

    print("[L4] Anti-Target-Leakage (no annotations, no CWE mentions, no comments)")
    leakage = check_leakage(test_files)
    l4_errors = len(errors) - l1_errors - l2_errors - l3_errors
    print("  Leakage instances: %d" % leakage)
    print("  Result: %d errors" % l4_errors)
    print()

    print("[L5] Scoring Pipeline Readiness")
    check_scoring_pipeline(csv_entries)
    l5_warnings = len(warnings)
    print("  Result: %d warnings" % l5_warnings)
    print()

    print("[L6] SARIF Integrity")
    sarif_path = PHP_DIR / "theauditor.sarif"
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
                    warnings.append("L6 theauditor.sarif is STALE (CSV hash mismatch)")
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
    print_report(csv_entries)

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
        print("RESULT: FAIL (%d errors)" % len(errors))
        return 1
    elif warnings:
        print("RESULT: PASS WITH WARNINGS (%d warnings)" % len(warnings))
        return 2
    else:
        print("RESULT: PASS")
        return 0


if __name__ == "__main__":
    sys.exit(main())
