#!/usr/bin/env python3
"""Ruby SAST Benchmark Fidelity Validator v1.0

Modeled after validate_php.py and TheAuditor's multi-level fidelity system:
  L1 -- Structural integrity (CSV <-> annotation cross-reference)
  L2 -- Roundtrip fidelity (file paths, line markers, snippet content)
  L3 -- Schema validation (required fields, valid CWEs, valid categories)
  L4 -- Semantic fidelity (vuln-line vs safe-line correctness, overlap detection)
  L5 -- Scoring pipeline readiness (RULE_MAP/SINK_MAP coverage in converter)

Exit 0 if all checks pass, 1 if any ERRORS, 2 if only WARNINGS.
No dependencies -- stdlib only.
"""

import hashlib
import json
import os
import re
import sys
from collections import defaultdict
from pathlib import Path

# ============================================================================
# Configuration
# ============================================================================
SCRIPT_DIR = Path(__file__).resolve().parent
BENCH_ROOT = SCRIPT_DIR.parent
RUBY_DIR = BENCH_ROOT / "ruby"
CSV_FILE = RUBY_DIR / "expectedresults-0.2.0.csv"
CONVERTER_PY = SCRIPT_DIR / "convert_theauditor.py"
SCAN_DIRS = [RUBY_DIR / "apps", RUBY_DIR / "testcode"]

PAT_START = re.compile(r"vuln-code-snippet\s+start\s+(\S+)")
PAT_END = re.compile(r"vuln-code-snippet\s+end\s+(\S+)")
PAT_VULN_LINE = re.compile(r"vuln-code-snippet\s+vuln-line\s+(\S+)")
PAT_SAFE_LINE = re.compile(r"vuln-code-snippet\s+safe-line\s+(\S+)")

REQUIRED_FIELDS = {"key", "category", "cwe", "vulnerable"}
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


# ============================================================================
# L1: Structural Integrity
# ============================================================================
def parse_csv():
    """Parse expectedresults CSV. Format: test name,category,real vulnerability,CWE"""
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


def scan_annotations():
    """Scan .rb files for vuln-code-snippet markers.
    Returns:
      annotations: {key: {file, start, end}}
      vuln_lines: {key: [(file, line_num), ...]}
      safe_lines: {key: [(file, line_num), ...]}
      file_lines: {rel_path: [line1, line2, ...]}  (cached for L2 checks)
    """
    annotations = {}
    open_snippets = {}
    vuln_lines = defaultdict(list)
    safe_lines = defaultdict(list)
    file_lines = {}

    for scan_dir in SCAN_DIRS:
        if not scan_dir.is_dir():
            continue
        for root, dirs, files in os.walk(scan_dir):
            dirs[:] = [d for d in dirs if d not in (
                ".git", "node_modules", ".auditor_venv", ".pf", "vendor", "target"
            )]
            for fn in sorted(files):
                if not fn.endswith(".rb"):
                    continue
                filepath = Path(root) / fn
                rel = str(filepath.relative_to(RUBY_DIR)).replace("\\", "/")
                try:
                    with open(filepath, "r", encoding="utf-8", errors="replace") as f:
                        lines = f.readlines()
                except OSError:
                    continue

                file_lines[rel] = lines

                for i, line in enumerate(lines, 1):
                    m = PAT_START.search(line)
                    if m:
                        key = m.group(1)
                        if key in open_snippets:
                            errors.append(
                                "L1 Duplicate annotation start for '%s' in %s:%d "
                                "(first at %s:%d)" % (key, rel, i, open_snippets[key][0], open_snippets[key][1])
                            )
                        open_snippets[key] = (rel, i)

                    m = PAT_END.search(line)
                    if m:
                        key = m.group(1)
                        if key in open_snippets:
                            start_file, start_line = open_snippets.pop(key)
                            if key in annotations:
                                errors.append("L1 Duplicate annotation key '%s' in %s:%d" % (key, rel, i))
                            annotations[key] = {"file": start_file, "start": start_line, "end": i}
                        else:
                            errors.append("L1 End without start for '%s' in %s:%d" % (key, rel, i))

                    m = PAT_VULN_LINE.search(line)
                    if m:
                        vuln_lines[m.group(1)].append((rel, i))

                    m = PAT_SAFE_LINE.search(line)
                    if m:
                        safe_lines[m.group(1)].append((rel, i))

    for key, (file, line) in open_snippets.items():
        errors.append("L1 Unclosed annotation start for '%s' in %s:%d" % (key, file, line))

    return annotations, vuln_lines, safe_lines, file_lines


# ============================================================================
# L2: Roundtrip Fidelity
# ============================================================================
def check_roundtrip(csv_entries, annotations):
    """Verify annotation files exist on disk and snippets are non-empty."""
    for key, info in csv_entries.items():
        if key not in annotations:
            continue

        ann = annotations[key]
        ann_file = ann["file"]

        full_path = RUBY_DIR / ann_file
        if not full_path.exists():
            errors.append("L2 File not found on disk: '%s' (key '%s')" % (ann_file, key))

        if ann["end"] - ann["start"] <= 1:
            warnings.append(
                "L2 Empty snippet for '%s' in %s:%d-%d "
                "(no code between start/end markers)" % (key, ann_file, ann["start"], ann["end"])
            )


# ============================================================================
# L3: Schema Validation
# ============================================================================
def check_schema(csv_entries):
    """Validate required fields, CWE values, and category consistency."""
    all_categories = set()

    for key, info in csv_entries.items():
        missing = REQUIRED_FIELDS - set(info.keys())
        if missing:
            errors.append(
                "L3 Missing required fields for '%s': %s "
                "(CSV line %s)" % (key, sorted(missing), info.get("_csv_line", "?"))
            )

        cwe = info.get("cwe")
        if cwe is not None and cwe not in VALID_CWES:
            warnings.append(
                "L3 CWE %d for key '%s' is not in the known CWE set "
                "(may be valid but not in our allowlist)" % (cwe, key)
            )

        cat = info.get("category")
        if cat:
            all_categories.add(cat)
            if cat not in VALID_CATEGORIES:
                warnings.append(
                    "L3 Category '%s' for key '%s' is not in the expected category set" % (cat, key)
                )

    return all_categories


# ============================================================================
# L4: Semantic Fidelity
# ============================================================================
def check_semantics(csv_entries, annotations, vuln_lines, safe_lines):
    """Verify vuln-line/safe-line markers match vulnerable classification."""
    for key, info in csv_entries.items():
        if key not in annotations:
            continue

        is_vuln = info.get("vulnerable", None)
        ann = annotations[key]
        has_vuln_marker = key in vuln_lines
        has_safe_marker = key in safe_lines

        if is_vuln is True:
            if not has_vuln_marker:
                errors.append(
                    "L4 Key '%s' is vulnerable=true but has NO vuln-line marker "
                    "in %s:%d-%d" % (key, ann["file"], ann["start"], ann["end"])
                )
            if has_safe_marker:
                errors.append(
                    "L4 Key '%s' is vulnerable=true but has a safe-line marker "
                    "(should be vuln-line)" % key
                )
        elif is_vuln is False:
            if not has_safe_marker:
                errors.append(
                    "L4 Key '%s' is vulnerable=false but has NO safe-line marker "
                    "in %s:%d-%d" % (key, ann["file"], ann["start"], ann["end"])
                )
            if has_vuln_marker:
                errors.append(
                    "L4 Key '%s' is vulnerable=false but has a vuln-line marker "
                    "(should be safe-line)" % key
                )

        for marker_list in [vuln_lines.get(key, []), safe_lines.get(key, [])]:
            for (mfile, mline) in marker_list:
                if mfile != ann["file"]:
                    errors.append(
                        "L4 Marker for '%s' at %s:%d is in a different file "
                        "than the snippet (%s)" % (key, mfile, mline, ann["file"])
                    )
                elif not (ann["start"] <= mline <= ann["end"]):
                    errors.append(
                        "L4 Marker for '%s' at %s:%d is OUTSIDE snippet range "
                        "%d-%d" % (key, mfile, mline, ann["start"], ann["end"])
                    )

    by_file = defaultdict(list)
    for key, ann in annotations.items():
        by_file[ann["file"]].append((ann["start"], ann["end"], key))

    for file, ranges in by_file.items():
        ranges.sort()
        for i in range(len(ranges) - 1):
            s1, e1, k1 = ranges[i]
            s2, e2, k2 = ranges[i + 1]
            if s2 <= e1:
                errors.append(
                    "L4 Overlapping snippets in %s: '%s' (%d-%d) "
                    "overlaps with '%s' (%d-%d)" % (file, k1, s1, e1, k2, s2, e2)
                )


# ============================================================================
# L5: Scoring Pipeline Readiness
# ============================================================================
def check_scoring_pipeline(csv_entries):
    """Verify all CWEs in the benchmark have coverage in convert_theauditor.py.

    The refactored converter uses CWE numbers directly (from pattern_findings.cwe
    and VULN_TYPE_TO_CWE). We check that every CWE in the CSV has at least one
    mapping in VULN_TYPE_TO_CWE, and that 'ruby' is in the annotation-language set.
    """
    if not CONVERTER_PY.exists():
        warnings.append("L5 convert_theauditor.py not found - cannot verify scoring pipeline")
        return

    with open(CONVERTER_PY, "r", encoding="utf-8") as f:
        converter_content = f.read()

    # Extract CWE numbers from VULN_TYPE_TO_CWE dict values
    mapped_cwes = set()
    for m in re.finditer(r":\s*(\d+)", converter_content):
        mapped_cwes.add(int(m.group(1)))

    # Collect unique CWEs from benchmark CSV
    benchmark_cwes = set()
    for key, info in csv_entries.items():
        cwe = info.get("cwe")
        if cwe and cwe > 0:
            benchmark_cwes.add(cwe)

    for cwe in sorted(benchmark_cwes):
        if cwe not in mapped_cwes:
            warnings.append(
                "L5 CWE %d exists in ground truth but has no VULN_TYPE_TO_CWE "
                "mapping in convert_theauditor.py (taint flows for this CWE "
                "won't be converted)" % cwe
            )

    # Verify Ruby is in the annotation-language set
    if '"ruby"' not in converter_content:
        warnings.append(
            "L5 'ruby' not found in convert_theauditor.py annotation-language set"
        )
    if '".rb"' not in converter_content:
        warnings.append(
            "L5 '.rb' extension not found in convert_theauditor.py extension map"
        )


# ============================================================================
# Report
# ============================================================================
def print_report(csv_entries, annotations):
    """Print the full fidelity report."""
    categories = defaultdict(lambda: {"tp": 0, "tn": 0})
    for key, info in csv_entries.items():
        cat = info.get("category", "unknown")
        if info.get("vulnerable", False):
            categories[cat]["tp"] += 1
        else:
            categories[cat]["tn"] += 1

    total_tp = sum(c["tp"] for c in categories.values())
    total_tn = sum(c["tn"] for c in categories.values())
    total = total_tp + total_tn

    print("CSV entries:    %d" % len(csv_entries))
    print("Annotations:    %d" % len(annotations))
    print("Match:          %s" % ("YES" if len(csv_entries) == len(annotations) else "NO - MISMATCH"))
    print("Total TP:       %d" % total_tp)
    print("Total TN:       %d" % total_tn)
    if total > 0:
        print("TP/TN split:    %.1f%% / %.1f%%" % (total_tp * 100.0 / total, total_tn * 100.0 / total))
    print()

    print("%-20s %5s %4s %4s %6s %8s" % ("Category", "CWE", "TP", "TN", "Total", "Balance"))
    print("-" * 52)

    cat_cwes = {}
    for key, info in csv_entries.items():
        cat = info.get("category")
        cwe = info.get("cwe")
        if cat and cwe:
            cat_cwes[cat] = cwe

    for cat in sorted(categories.keys()):
        tp = categories[cat]["tp"]
        tn = categories[cat]["tn"]
        cat_total = tp + tn
        cwe = cat_cwes.get(cat, "?")
        if cat_total > 0:
            balance = "%d/%d" % (tp * 100 // cat_total, tn * 100 // cat_total)
        else:
            balance = "N/A"
        print("%-20s %5s %4d %4d %6d %8s" % (cat, cwe, tp, tn, cat_total, balance))

    print("-" * 52)
    balance_str = "%d/%d" % (total_tp * 100 // total, total_tn * 100 // total) if total > 0 else "N/A"
    print("%-20s %5s %4d %4d %6d %8s" % ("TOTAL", "", total_tp, total_tn, total, balance_str))
    print()


def main():
    print("=" * 64)
    print("  Ruby SAST Benchmark Fidelity Validator v1.0")
    print("  Modeled after TheAuditor fidelity system (L1-L5)")
    print("=" * 64)
    print()

    # --- L1: Structural Integrity ---
    print("[L1] Structural Integrity (CSV <-> annotation cross-reference)")
    csv_entries = parse_csv()
    annotations, vuln_lines, safe_lines, file_lines = scan_annotations()

    csv_keys = set(csv_entries.keys())
    ann_keys = set(annotations.keys())

    for key in sorted(csv_keys - ann_keys):
        errors.append("L1 Orphan CSV: '%s' in ground truth but no annotation in source" % key)
    for key in sorted(ann_keys - csv_keys):
        errors.append("L1 Orphan annotation: '%s' in source but no CSV entry" % key)

    l1_errors = len(errors)
    print("  Checks: duplicate keys, orphans, balanced start/end, unclosed snippets")
    print("  Result: %d errors found" % l1_errors)
    print()

    # --- L2: Roundtrip Fidelity ---
    print("[L2] Roundtrip Fidelity (file existence, empty snippets)")
    check_roundtrip(csv_entries, annotations)
    l2_errors = len(errors) - l1_errors
    print("  Checks: annotation source files exist on disk, snippets non-empty")
    print("  Result: %d errors found" % l2_errors)
    print()

    # --- L3: Schema Validation ---
    print("[L3] Schema Validation (required fields, valid CWEs, valid categories)")
    all_categories = check_schema(csv_entries)
    l3_errors = len(errors) - l1_errors - l2_errors
    print("  Checks: 4 CSV columns present, CWE in valid set, category in valid set")
    print("  Categories found: %d (%s)" % (len(all_categories), ", ".join(sorted(all_categories))))
    print("  Result: %d errors found" % l3_errors)
    print()

    # --- L4: Semantic Fidelity ---
    print("[L4] Semantic Fidelity (vuln-line/safe-line correctness, overlap detection)")
    check_semantics(csv_entries, annotations, vuln_lines, safe_lines)
    l4_errors = len(errors) - l1_errors - l2_errors - l3_errors
    print("  Checks: TP has vuln-line, TN has safe-line, markers inside snippet, no overlaps")
    print("  Result: %d errors found" % l4_errors)
    print()

    # --- L5: Scoring Pipeline Readiness ---
    print("[L5] Scoring Pipeline Readiness (CWE coverage + annotation support)")
    check_scoring_pipeline(csv_entries)
    l5_warnings = len(warnings)
    print("  Checks: every CWE has VULN_TYPE_TO_CWE mapping, Ruby in annotation set")
    print("  Result: %d warnings" % l5_warnings)
    print()

    # --- L6: SARIF Integrity ---
    print("[L6] SARIF Integrity (theauditor.sarif freshness)")
    sarif_path = RUBY_DIR / "theauditor.sarif"
    if not sarif_path.exists():
        print("  theauditor.sarif not found -- skipping (run converter to generate)")
    else:
        try:
            with open(sarif_path, "r", encoding="utf-8") as sf:
                sarif_data = json.load(sf)
            integrity = sarif_data.get("runs", [{}])[0].get("properties", {}).get("integrity")
            if integrity is None:
                warnings.append("L6 theauditor.sarif has no integrity metadata (legacy file, regenerate with converter v2.0+)")
            else:
                h = hashlib.sha256()
                with open(CSV_FILE, "rb") as cf:
                    for chunk in iter(lambda: cf.read(65536), b""):
                        h.update(chunk)
                if h.hexdigest() != integrity.get("csv_sha256"):
                    warnings.append("L6 theauditor.sarif is STALE: CSV hash mismatch (re-run convert_theauditor.py)")
                else:
                    print("  SARIF integrity: CURRENT (CSV hash matches)")
        except (json.JSONDecodeError, OSError) as e:
            warnings.append("L6 Could not read theauditor.sarif: %s" % e)
    l6_warnings = len(warnings) - l5_warnings
    print("  Result: %d warnings" % l6_warnings)
    print()

    # --- Summary ---
    print("=" * 64)
    print("  SUMMARY")
    print("=" * 64)
    print()

    print_report(csv_entries, annotations)

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
        print("  %d errors must be fixed before benchmark is valid." % len(errors))
        return 1
    elif warnings:
        print("RESULT: PASS WITH WARNINGS")
        print("  %d warnings found. Review recommended but not blocking." % len(warnings))
        return 2
    else:
        print("RESULT: PASS")
        print("  All L1-L5 fidelity checks passed. Benchmark is valid.")
        return 0


if __name__ == "__main__":
    sys.exit(main())
