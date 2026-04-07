#!/usr/bin/env python3
"""Score a SAST tool's SARIF output against the Go/Rust/Bash/PHP/Ruby SAST Benchmark.

Usage:
    python score_sarif.py <tool_output.sarif> [expectedresults.csv]

Accepts any SARIF 2.1.0 file. A test case is considered "detected" if the
tool produced at least one finding whose location URI contains the test
file name (e.g., benchmark_test_00042.go).

Two scoring methods are computed:

  Category-Averaged (OWASP Standard):
    1. Compute TPR and FPR independently for each category
    2. Average all category TPRs for overall TPR
    3. Average all category FPRs for overall FPR
    4. Score = averaged_TPR - averaged_FPR
    This prevents large categories from dominating small ones.

  Flat Aggregate (for comparison):
    Sum all TP/FP/FN/TN across all tests, compute global TPR and FPR.

Zero external dependencies -- stdlib only.
"""

import hashlib
import json
import os
import re
import sys
from collections import defaultdict


def load_expected_results(csv_path):
    """Load ground truth CSV into {test_name: {category, vulnerable, cwe}}."""
    expected = {}
    with open(csv_path, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            parts = line.split(",")
            if len(parts) < 4:
                continue
            name = parts[0].strip()
            expected[name] = {
                "category": parts[1].strip(),
                "vulnerable": parts[2].strip().lower() == "true",
                "cwe": int(parts[3].strip()),
            }
    return expected


def extract_test_name_from_uri(uri):
    """Extract test case key from a SARIF URI.

    Handles:
      testcode/benchmark_test_00001.go  (Go — lowercase with underscores)
      testcode/BenchmarkTest00001.py    (Python/Java — CamelCase)
      /abs/path/testcode/benchmark_test_00001.go
      C:\\Users\\...\\BenchmarkTest00001.py
      scenarios/scenario_0142/variant_a/app.py  (Chains — directory-based)
    """
    # Chains: directory-based (scenario_NNNN/)
    m = re.search(r"scenario_(\d{4})/", uri)
    if m:
        return "ChainScenario%s" % m.group(1)
    # Go/Rust/Bash/PHP: lowercase with underscores
    m = re.search(r"benchmark_test_(\d{5})\.\w+", uri)
    if m:
        return "BenchmarkTest" + m.group(1)
    # Python/Java: CamelCase
    m = re.search(r"BenchmarkTest(\d{5})\.\w+", uri)
    if m:
        return "BenchmarkTest" + m.group(1)
    return None


def load_sarif_detections_filename(sarif_path):
    """Parse SARIF and return set of detected test names.

    Two matching modes (both checked for every result):
      1. URI-based: extracts BenchmarkTestNNNNN from file paths (Go)
      2. Key-based: reads properties.testCaseKey from result (Bash/Rust)
    """
    with open(sarif_path, "r", encoding="utf-8") as f:
        sarif = json.load(f)

    detected = set()

    for run in sarif.get("runs", []):
        for result in run.get("results", []):
            # Mode 1: URI-based matching (Go — one file per test case)
            locations = result.get("locations", [])
            for loc in locations:
                phys = loc.get("physicalLocation", {})
                artifact = phys.get("artifactLocation", {})
                uri = artifact.get("uri", "")
                if uri:
                    name = extract_test_name_from_uri(uri)
                    if name:
                        detected.add(name)

            # Mode 2: Key-based matching (Bash/Rust — annotation-resolved)
            props = result.get("properties", {})
            tc_key = props.get("testCaseKey")
            if tc_key:
                detected.add(tc_key)

    return detected


def load_sarif_findings(sarif_path):
    """Parse SARIF and return list of (file, line, ruleId) tuples for all findings."""
    with open(sarif_path, "r", encoding="utf-8") as f:
        sarif = json.load(f)

    findings = []
    for run in sarif.get("runs", []):
        for result in run.get("results", []):
            rule_id = result.get("ruleId", "")
            locations = result.get("locations", [])
            for loc in locations:
                phys = loc.get("physicalLocation", {})
                artifact = phys.get("artifactLocation", {})
                uri = artifact.get("uri", "").replace("\\", "/")
                region = phys.get("region", {})
                line = region.get("startLine", 0)
                if uri and line:
                    findings.append((uri, line, rule_id))

    return findings


def scan_annotations(source_dirs):
    """Scan source files for vuln-code-snippet annotations.

    Returns dict of {key: {"file": relative_path, "start": line, "end": line}}.
    Works for Rust (.rs), Bash (.sh), PHP (.php), and Ruby (.rb) annotation-based benchmarks.
    """
    pat_start = re.compile(r"vuln-code-snippet\s+start\s+(\S+)")
    pat_end = re.compile(r"vuln-code-snippet\s+end\s+(\S+)")

    annotations = {}
    for source_dir in source_dirs:
        for root, dirs, files in os.walk(source_dir):
            dirs[:] = [d for d in dirs if d not in ("target", ".git", "node_modules", ".auditor_venv")]
            for fn in files:
                if not (fn.endswith(".rs") or fn.endswith(".sh") or fn.endswith(".php") or fn.endswith(".rb") or fn.endswith(".py") or fn.endswith(".js") or fn.endswith(".go")):
                    continue
                fpath = os.path.join(root, fn)
                try:
                    with open(fpath, "r", encoding="utf-8", errors="replace") as f:
                        lines = f.readlines()
                except Exception:
                    continue

                rel = os.path.relpath(fpath, os.path.dirname(source_dir)).replace("\\", "/")
                opens = {}
                for i, line in enumerate(lines, 1):
                    m = pat_start.search(line)
                    if m:
                        opens[m.group(1)] = i
                    m = pat_end.search(line)
                    if m:
                        key = m.group(1)
                        if key in opens:
                            annotations[key] = {"file": rel, "start": opens.pop(key), "end": i}

    return annotations


def detect_annotation_mode(expected):
    """Return True if CSV keys use annotation-based identity (not filename-based)."""
    for name in expected:
        if re.match(r"BenchmarkTest\d{5}$", name):
            return False
        if re.match(r"ChainScenario\d{4}$", name):
            return False
    return True


def compute_detections_annotation(expected, findings, annotations):
    """Match SARIF findings against annotation line ranges.

    A test case is "detected" if a SARIF finding's (file, line) falls
    within the annotation's (file, start_line, end_line) range AND the
    finding's ruleId CWE matches the test case's expected CWE.
    This follows the OWASP standard: correct CWE classification required.
    """
    detected = set()

    # Build lookup: normalized_file -> list of (line, ruleId)
    finding_map = defaultdict(list)
    for uri, line, rule_id in findings:
        # Normalize: strip leading paths to get relative
        normalized = uri.replace("\\", "/")
        finding_map[normalized].append((line, rule_id))

    for key, info in expected.items():
        ann = annotations.get(key)
        if not ann:
            continue

        expected_cwe = info["cwe"]
        ann_file = ann["file"]
        ann_start = ann["start"]
        ann_end = ann["end"]

        # Check all path variants for the annotation file
        for fpath, line_rules in finding_map.items():
            if fpath.endswith(ann_file) or ann_file.endswith(fpath) or fpath == ann_file:
                for line, rule_id in line_rules:
                    if ann_start <= line <= ann_end:
                        # CWE-aware: strip "taint:" prefix, parse CWE number
                        cwe_str = rule_id.replace("taint:", "")
                        try:
                            finding_cwe = int(cwe_str)
                        except (ValueError, TypeError):
                            continue
                        if finding_cwe == expected_cwe:
                            detected.add(key)
                            break
                if key in detected:
                    break

    return detected


def compute_detections_filename_cataware(expected, sarif_path):
    """Filename-based matching with CWE awareness (OWASP standard).

    A test case is "detected" only if a SARIF finding references its filename
    AND the finding's ruleId CWE matches the test case's expected CWE.
    SARIF ruleId is a CWE number (e.g., "89" or "taint:89").
    """
    with open(sarif_path, "r", encoding="utf-8") as f:
        sarif = json.load(f)

    test_cwes = {name: info["cwe"] for name, info in expected.items()}
    detected = set()

    for run in sarif.get("runs", []):
        for result in run.get("results", []):
            rule_id = result.get("ruleId", "")
            # Strip "taint:" prefix, parse CWE number
            cwe_str = rule_id.replace("taint:", "")
            try:
                finding_cwe = int(cwe_str)
            except (ValueError, TypeError):
                continue

            for loc in result.get("locations", []):
                uri = loc.get("physicalLocation", {}).get("artifactLocation", {}).get("uri", "")
                if uri:
                    name = extract_test_name_from_uri(uri)
                    if name and name in test_cwes and finding_cwe == test_cwes[name]:
                        detected.add(name)

    return detected


def compute_scores(expected, detected):
    """Compute per-category and overall scores."""
    categories = sorted(set(e["category"] for e in expected.values()))

    per_cat = {}
    for cat in categories:
        tests = {k: v for k, v in expected.items() if v["category"] == cat}
        tp = fp = fn = tn = 0
        for name, info in tests.items():
            is_detected = name in detected
            if info["vulnerable"] and is_detected:
                tp += 1
            elif info["vulnerable"] and not is_detected:
                fn += 1
            elif not info["vulnerable"] and is_detected:
                fp += 1
            else:
                tn += 1

        total_real = tp + fn
        total_safe = fp + tn
        tpr = tp / total_real if total_real > 0 else 0.0
        fpr = fp / total_safe if total_safe > 0 else 0.0

        cwe = list(tests.values())[0]["cwe"] if tests else 0
        per_cat[cat] = {
            "cwe": cwe,
            "tp": tp, "fp": fp, "fn": fn, "tn": tn,
            "tpr": tpr, "fpr": fpr, "score": tpr - fpr,
        }

    return per_cat


def print_flat_scoring(per_cat):
    """Print flat aggregate scoring table."""
    print("=" * 78)
    print("FLAT AGGREGATE SCORING")
    print("=" * 78)
    print()
    header = "%-16s %-6s %-5s %-5s %-5s %-5s %7s %7s %7s" % (
        "Category", "CWE", "TP", "FP", "FN", "TN", "TPR", "FPR", "Score")
    print(header)
    print("-" * 78)

    t_tp = t_fp = t_fn = t_tn = 0
    for cat in sorted(per_cat.keys()):
        s = per_cat[cat]
        t_tp += s["tp"]
        t_fp += s["fp"]
        t_fn += s["fn"]
        t_tn += s["tn"]
        print("%-16s %-6d %-5d %-5d %-5d %-5d %6.1f%% %6.1f%% %+6.1f%%" % (
            cat, s["cwe"], s["tp"], s["fp"], s["fn"], s["tn"],
            s["tpr"] * 100, s["fpr"] * 100, s["score"] * 100))

    total_real = t_tp + t_fn
    total_safe = t_fp + t_tn
    flat_tpr = t_tp / total_real if total_real > 0 else 0.0
    flat_fpr = t_fp / total_safe if total_safe > 0 else 0.0
    flat_score = flat_tpr - flat_fpr

    print("-" * 78)
    print("%-16s %-6s %-5d %-5d %-5d %-5d %6.1f%% %6.1f%% %+6.1f%%" % (
        "TOTAL", "", t_tp, t_fp, t_fn, t_tn,
        flat_tpr * 100, flat_fpr * 100, flat_score * 100))
    print()

    return flat_tpr, flat_fpr, flat_score


def print_category_averaged_scoring(per_cat):
    """Print OWASP category-averaged scoring table."""
    print("=" * 78)
    print("CATEGORY-AVERAGED SCORING (OWASP Standard)")
    print("=" * 78)
    print()
    print("Each category is weighted equally regardless of test count.")
    print("This is the official OWASP Benchmark scoring methodology.")
    print()

    header = "%-16s %-6s %7s %7s %7s" % (
        "Category", "CWE", "TPR", "FPR", "Score")
    print(header)
    print("-" * 50)

    sum_tpr = 0.0
    sum_fpr = 0.0
    n_cats = len(per_cat)

    for cat in sorted(per_cat.keys()):
        s = per_cat[cat]
        sum_tpr += s["tpr"]
        sum_fpr += s["fpr"]
        print("%-16s %-6d %6.1f%% %6.1f%% %+6.1f%%" % (
            cat, s["cwe"],
            s["tpr"] * 100, s["fpr"] * 100, s["score"] * 100))

    avg_tpr = sum_tpr / n_cats if n_cats > 0 else 0.0
    avg_fpr = sum_fpr / n_cats if n_cats > 0 else 0.0
    avg_score = avg_tpr - avg_fpr

    print("-" * 50)
    print("%-16s %-6s %6.1f%% %6.1f%% %+6.1f%%" % (
        "MACRO AVERAGE", "", avg_tpr * 100, avg_fpr * 100, avg_score * 100))
    print()

    return avg_tpr, avg_fpr, avg_score


def print_summary(expected, detected, flat_score, avg_score):
    """Print detection summary."""
    total_tests = len(expected)
    total_detected = len(detected & set(expected.keys()))
    total_vuln = sum(1 for e in expected.values() if e["vulnerable"])
    total_safe = total_tests - total_vuln

    print("=" * 78)
    print("SUMMARY")
    print("=" * 78)
    print()
    print("  Total test cases:       %d" % total_tests)
    print("  Vulnerable (ground truth): %d" % total_vuln)
    print("  Safe (ground truth):    %d" % total_safe)
    print("  TP/TN balance:          %d/%d" % (
        total_vuln * 100 // total_tests,
        total_safe * 100 // total_tests))
    print("  Tests with findings:    %d" % total_detected)
    print()
    print("  Flat Score:             %+.1f%%" % (flat_score * 100))
    print("  Category-Averaged:      %+.1f%% (OWASP standard)" % (avg_score * 100))
    print()
    print("  Score interpretation:")
    print("    +100%%  Perfect (catches everything, zero false alarms)")
    print("       0%%  Random guessing (no better than a coin flip)")
    print("    -100%%  Perfectly wrong (flags safe, misses vulnerable)")
    print()


def main():
    # Parse args: score_sarif.py <sarif> [csv] [--annotations-dir <dir> ...]
    args = sys.argv[1:]
    annotations_dirs = []
    positional = []

    i = 0
    while i < len(args):
        if args[i] == "--annotations-dir" and i + 1 < len(args):
            annotations_dirs.append(args[i + 1])
            i += 2
        elif args[i].startswith("--"):
            i += 1
        else:
            positional.append(args[i])
            i += 1

    if len(positional) < 1:
        print("Usage: python score_sarif.py <tool_output.sarif> [expectedresults.csv] [--annotations-dir <dir>]")
        print()
        print("Arguments:")
        print("  tool_output.sarif      SARIF 2.1.0 file from any SAST tool")
        print("  expectedresults.csv    Ground truth CSV (default: auto-detect)")
        print("  --annotations-dir DIR  Extra source directory with vuln-code-snippet annotations")
        print("                         (apps/ and testcode/ next to CSV are always auto-included)")
        print()
        print("Examples:")
        print("  # Annotation-based (Rust/Bash/PHP/Ruby) -- simplest, auto-detects dirs:")
        print("  python score_sarif.py theauditor.sarif php/expectedresults-0.3.2.csv")
        print()
        print("  # Go (filename-based matching, no annotations needed):")
        print("  python score_sarif.py results.sarif go/expectedresults-0.5.1.csv")
        sys.exit(1)

    sarif_path = positional[0]

    if len(positional) >= 2:
        csv_path = positional[1]
    else:
        script_dir = os.path.dirname(os.path.abspath(__file__))
        csv_path = os.path.join(script_dir, "..", "go", "expectedresults-0.3.2.csv")

    if not os.path.isfile(sarif_path):
        print("Error: SARIF file not found: %s" % sarif_path, file=sys.stderr)
        sys.exit(1)

    if not os.path.isfile(csv_path):
        print("Error: Expected results CSV not found: %s" % csv_path, file=sys.stderr)
        sys.exit(1)

    expected = load_expected_results(csv_path)

    # Determine matching mode
    use_annotations = annotations_dirs or detect_annotation_mode(expected)

    if use_annotations:
        # Always ensure apps/ and testcode/ next to CSV are included.
        # Passing --annotations-dir for only ONE of them (e.g. testcode
        # but not apps) silently drops test cases and produces wrong scores.
        csv_dir = os.path.dirname(os.path.abspath(csv_path))
        existing = set(os.path.abspath(d) for d in annotations_dirs)
        for subdir in ["apps", "testcode", "scenarios"]:
            candidate = os.path.join(csv_dir, subdir)
            if os.path.isdir(candidate) and os.path.abspath(candidate) not in existing:
                annotations_dirs.append(candidate)

    # Check SARIF integrity if it has TheAuditor metadata
    try:
        with open(sarif_path, "r", encoding="utf-8") as _sf:
            _sarif_data = json.load(_sf)
        _integrity = _sarif_data.get("runs", [{}])[0].get("properties", {}).get("integrity")
        if _integrity and _integrity.get("csv_sha256"):
            _h = hashlib.sha256()
            with open(csv_path, "rb") as _cf:
                for _chunk in iter(lambda: _cf.read(65536), b""):
                    _h.update(_chunk)
            if _h.hexdigest() != _integrity["csv_sha256"]:
                print("WARNING: SARIF was generated against a different CSV version.",
                      file=sys.stderr)
                print("  SARIF CSV hash: %s" % _integrity["csv_sha256"][:16], file=sys.stderr)
                print("  Current CSV:    %s" % _h.hexdigest()[:16], file=sys.stderr)
                print("  Re-run convert_theauditor.py to regenerate.", file=sys.stderr)
                print(file=sys.stderr)
    except (json.JSONDecodeError, OSError):
        pass

    if use_annotations:
        print("Mode: annotation-based matching (Rust/Bash/PHP/Ruby)")
        annotations = scan_annotations(annotations_dirs)
        print("  Annotations found: %d" % len(annotations))
        findings = load_sarif_findings(sarif_path)
        print("  SARIF findings: %d" % len(findings))
        detected = compute_detections_annotation(expected, findings, annotations)
    else:
        print("Mode: filename-based matching with CWE awareness")
        detected = compute_detections_filename_cataware(expected, sarif_path)

    print()
    per_cat = compute_scores(expected, detected)

    avg_tpr, avg_fpr, avg_score = print_category_averaged_scoring(per_cat)
    flat_tpr, flat_fpr, flat_score = print_flat_scoring(per_cat)
    print_summary(expected, detected, flat_score, avg_score)


if __name__ == "__main__":
    main()
