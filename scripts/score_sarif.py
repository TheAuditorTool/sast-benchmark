#!/usr/bin/env python3
"""Score a SAST tool's SARIF output against the Go/Rust/Bash SAST Benchmark.

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
    """Extract BenchmarkTestNNNNN from a SARIF URI.

    Handles:
      testcode/benchmark_test_00001.go
      /abs/path/testcode/benchmark_test_00001.go
      C:\\Users\\...\\benchmark_test_00001.go
      benchmark_test_00001.rs
      benchmark_test_00001.sh
    """
    m = re.search(r"benchmark_test_(\d{5})\.\w+", uri)
    if m:
        return "BenchmarkTest" + m.group(1)
    return None


def load_sarif_detections(sarif_path):
    """Parse SARIF and return set of detected test names."""
    with open(sarif_path, "r", encoding="utf-8") as f:
        sarif = json.load(f)

    detected = set()

    for run in sarif.get("runs", []):
        for result in run.get("results", []):
            locations = result.get("locations", [])
            for loc in locations:
                phys = loc.get("physicalLocation", {})
                artifact = phys.get("artifactLocation", {})
                uri = artifact.get("uri", "")
                if uri:
                    name = extract_test_name_from_uri(uri)
                    if name:
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
    if len(sys.argv) < 2:
        print("Usage: python score_sarif.py <tool_output.sarif> [expectedresults.csv]")
        print()
        print("Arguments:")
        print("  tool_output.sarif    SARIF 2.1.0 file from any SAST tool")
        print("  expectedresults.csv  Ground truth CSV (default: ../go/expectedresults-0.1.csv)")
        sys.exit(1)

    sarif_path = sys.argv[1]

    if len(sys.argv) >= 3:
        csv_path = sys.argv[2]
    else:
        script_dir = os.path.dirname(os.path.abspath(__file__))
        csv_path = os.path.join(script_dir, "..", "go", "expectedresults-0.1.csv")

    if not os.path.isfile(sarif_path):
        print("Error: SARIF file not found: %s" % sarif_path, file=sys.stderr)
        sys.exit(1)

    if not os.path.isfile(csv_path):
        print("Error: Expected results CSV not found: %s" % csv_path, file=sys.stderr)
        sys.exit(1)

    expected = load_expected_results(csv_path)
    detected = load_sarif_detections(sarif_path)

    print()
    per_cat = compute_scores(expected, detected)

    avg_tpr, avg_fpr, avg_score = print_category_averaged_scoring(per_cat)
    flat_tpr, flat_fpr, flat_score = print_flat_scoring(per_cat)
    print_summary(expected, detected, flat_score, avg_score)


if __name__ == "__main__":
    main()
