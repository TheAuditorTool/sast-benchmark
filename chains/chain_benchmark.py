#!/usr/bin/env python3
"""
Chain Detection Benchmark Scoring Script
=========================================
Scores SAST tools against expectedresults-0.1.0.csv for compound exploit
chain detection.

Unlike traditional SAST benchmarks that test individual vulnerability
detection, this benchmark tests whether a tool can correlate multiple
findings across files into compound exploit paths whose severity exceeds
any individual component.

Usage:
    python3 chain_benchmark.py

Reads findings from .pf/repo_index.db and compares against ground truth.
A finding counts as a chain detection ONLY if it maps to a chain-specific
category. Standard individual findings (sqli, cmdi, etc.) do NOT count.

Scoring: TPR - FPR (Youden's J). 0% = random guessing. +100% = perfect.
"""
import os
import re
import sqlite3
from collections import defaultdict
from pathlib import Path

# ============================================================================
# Configuration
# ============================================================================
BENCHMARK_ROOT = Path(os.path.dirname(os.path.abspath(__file__)))
DB_PATH = BENCHMARK_ROOT / ".pf" / "repo_index.db"
GROUND_TRUTH_PATH = BENCHMARK_ROOT / "expectedresults-0.1.0.csv"
SCENARIOS_DIR = BENCHMARK_ROOT / "scenarios"

# Chain-specific rule -> benchmark category mapping
# Standard SAST rules (sqli, cmdi, etc.) are NOT mapped here.
# Only rules that indicate the tool identified a CHAIN are counted.
CHAIN_RULE_MAP = {
    "chain-unauth-injection": "unauth_injection",
    "chain-unauth-sqli": "unauth_injection",
    "chain-unauth-cmdi": "unauth_injection",
    "chain-ssrf-pivot": "ssrf_pivot",
    "chain-ssrf-internal": "ssrf_pivot",
    "chain-ssrf-metadata": "ssrf_pivot",
    "chain-compound-injection": "compound_injection",
    "chain-second-order-xss": "compound_injection",
    "chain-deser-to-rce": "compound_injection",
    "chain-multi-stage": "multi_stage",
    "chain-upload-rce": "multi_stage",
    "chain-log-xss": "multi_stage",
}

# Chain-specific taint vulnerability_type -> category
CHAIN_SINK_MAP = {
    "Unauthenticated Injection": "unauth_injection",
    "SSRF to Internal Service": "ssrf_pivot",
    "Second-Order Injection": "compound_injection",
    "Multi-Stage RCE": "multi_stage",
}

# Rules to ignore (standard individual findings, not chain detections)
INDIVIDUAL_RULES = {
    "sqli", "cmdi", "pathtraver", "xss", "ssrf", "deserial",
    "fileupload", "loginjection", "weakrand", "weakhash",
}

# ============================================================================
# CSV Parser
# ============================================================================
def parse_ground_truth(path):
    """Parse expectedresults CSV."""
    test_cases = []
    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            stripped = line.strip()
            if not stripped or stripped.startswith("#"):
                continue
            parts = stripped.split(",")
            if len(parts) < 4:
                continue
            test_cases.append({
                "key": parts[0].strip(),
                "category": parts[1].strip(),
                "vulnerable": parts[2].strip().lower() == "true",
                "cwe": int(parts[3].strip()),
            })
    return test_cases


# ============================================================================
# Source File Scanner
# ============================================================================
def scan_annotations(root_dir):
    """Scan scenario source files for vuln-code-snippet annotations."""
    snippets = {}
    pat_start = re.compile(r"vuln-code-snippet\s+start\s+(\S+)")
    pat_end = re.compile(r"vuln-code-snippet\s+end\s+(\S+)")

    for dirpath, dirs, filenames in os.walk(root_dir):
        dirs[:] = [d for d in dirs if d not in (".git", "__pycache__", ".pf", "node_modules")]
        for fn in filenames:
            if fn.endswith((".py", ".js", ".go", ".html", ".json")):
                fp = os.path.join(dirpath, fn)
                rel = os.path.relpath(fp, root_dir).replace("\\", "/")

                try:
                    with open(fp, "r", encoding="utf-8", errors="replace") as f:
                        lines = f.readlines()
                except OSError:
                    continue

                opens = {}
                for i, ln in enumerate(lines, 1):
                    m = pat_start.search(ln)
                    if m:
                        opens[m.group(1)] = i

                    m = pat_end.search(ln)
                    if m:
                        key = m.group(1)
                        if key in opens:
                            if key not in snippets:
                                snippets[key] = []
                            snippets[key].append({
                                "file": rel,
                                "start": opens.pop(key),
                                "end": i,
                            })

    return snippets


# ============================================================================
# Scoring
# ============================================================================
def main():
    test_cases = parse_ground_truth(str(GROUND_TRUTH_PATH))
    print(f"Loaded {len(test_cases)} chain test cases from ground truth")

    snippets = scan_annotations(str(BENCHMARK_ROOT))
    print(f"Found {len(snippets)} annotated chain endpoints in source files")

    # Check coverage
    missing = [tc["key"] for tc in test_cases if tc["key"] not in snippets]
    if missing:
        print(f"\nWARNING: {len(missing)} test cases have no source annotation:")
        for k in missing:
            print(f"  - {k}")
        print()

    # Load findings
    if not DB_PATH.exists():
        print(f"\nERROR: Database not found at {DB_PATH}")
        print("Run your SAST tool on the benchmark scenarios first.")
        print("\nShowing ground truth summary:\n")
        show_summary(test_cases)
        return

    conn = sqlite3.connect(str(DB_PATH))
    c = conn.cursor()
    findings = defaultdict(set)

    # Track 1: Chain-specific pattern findings
    try:
        c.execute("SELECT file, line, rule FROM pattern_findings")
        for f, ln, r in c.fetchall():
            cat = CHAIN_RULE_MAP.get(r)
            if cat:
                findings[f].add((ln, cat))
    except sqlite3.OperationalError:
        pass

    # Track 2: Chain-specific taint flows
    try:
        c.execute(
            "SELECT sink_file, sink_line, vulnerability_type "
            "FROM resolved_flow_audit WHERE status = 'VULNERABLE'"
        )
        for f, ln, vt in c.fetchall():
            cat = CHAIN_SINK_MAP.get(vt)
            if cat:
                findings[f].add((ln, cat))
    except sqlite3.OperationalError:
        pass

    # Track 3: Dedicated chain findings table (if tool has one)
    try:
        c.execute(
            "SELECT endpoint_file, endpoint_line, chain_category "
            "FROM chain_findings"
        )
        for f, ln, cat in c.fetchall():
            findings[f].add((ln, cat))
    except sqlite3.OperationalError:
        pass

    conn.close()

    # Score per category
    cats = sorted(set(tc["category"] for tc in test_cases))
    results = {}

    for cat in cats:
        tp = fp = fn = tn = 0
        cat_cases = [tc for tc in test_cases if tc["category"] == cat]

        for tc in cat_cases:
            key = tc["key"]
            is_vulnerable = tc["vulnerable"]
            detected = False

            locs = snippets.get(key, [])
            for loc in locs:
                file_findings = findings.get(loc["file"], set())
                for ln, found_cat in file_findings:
                    if loc["start"] <= ln <= loc["end"] and found_cat == cat:
                        detected = True
                        break
                if detected:
                    break

            if is_vulnerable:
                if detected:
                    tp += 1
                else:
                    fn += 1
            else:
                if detected:
                    fp += 1
                else:
                    tn += 1

        results[cat] = {"tp": tp, "fp": fp, "fn": fn, "tn": tn}
        cwe = cat_cases[0].get("cwe", 0) if cat_cases else 0
        results[cat]["cwe"] = cwe

    # Print scorecard
    print()
    print(
        "%-22s %-6s %-5s %-5s %-5s %-5s %7s %7s %7s"
        % ("Category", "CWE", "TP", "FP", "FN", "TN", "TPR", "FPR", "Score")
    )
    print("-" * 80)

    total_tp = total_fp = total_fn = total_tn = 0

    for cat in cats:
        r = results[cat]
        tp, fp, fn, tn = r["tp"], r["fp"], r["fn"], r["tn"]
        cwe = r["cwe"]
        total_tp += tp
        total_fp += fp
        total_fn += fn
        total_tn += tn

        tr = tp + fn
        ts = fp + tn
        tpr = tp / tr if tr else 0
        fpr = fp / ts if ts else 0
        score = tpr - fpr

        print(
            "%-22s %-6d %-5d %-5d %-5d %-5d %6.1f%% %6.1f%% %+6.1f%%"
            % (cat, cwe, tp, fp, fn, tn, tpr * 100, fpr * 100, score * 100)
        )

    overall_tpr = total_tp / (total_tp + total_fn) if (total_tp + total_fn) else 0
    overall_fpr = total_fp / (total_fp + total_tn) if (total_fp + total_tn) else 0
    overall_score = overall_tpr - overall_fpr

    print("-" * 80)
    print(
        "%-22s %-6s %-5d %-5d %-5d %-5d %6.1f%% %6.1f%% %+6.1f%%"
        % (
            "OVERALL", "", total_tp, total_fp, total_fn, total_tn,
            overall_tpr * 100, overall_fpr * 100, overall_score * 100,
        )
    )

    # FN analysis
    print("\n\n=== FALSE NEGATIVES (Missed Chain Detections) ===\n")
    fn_count = 0
    for tc in test_cases:
        if not tc["vulnerable"]:
            continue
        key = tc["key"]
        cat = tc["category"]
        locs = snippets.get(key, [])
        detected = False
        for loc in locs:
            file_findings = findings.get(loc["file"], set())
            for ln, found_cat in file_findings:
                if loc["start"] <= ln <= loc["end"] and found_cat == cat:
                    detected = True
                    break
            if detected:
                break
        if not detected:
            fn_count += 1
            loc_info = locs[0] if locs else {"file": "?", "start": "?", "end": "?"}
            print(f"  FN: {key} [{cat}] {loc_info['file']}:{loc_info['start']}-{loc_info['end']}")

    if fn_count == 0:
        print("  None! All exploit chains detected.")

    # FP analysis
    print("\n\n=== FALSE POSITIVES (Safe Chains Incorrectly Flagged) ===\n")
    fp_count = 0
    for tc in test_cases:
        if tc["vulnerable"]:
            continue
        key = tc["key"]
        cat = tc["category"]
        locs = snippets.get(key, [])
        detected = False
        for loc in locs:
            file_findings = findings.get(loc["file"], set())
            for ln, found_cat in file_findings:
                if loc["start"] <= ln <= loc["end"] and found_cat == cat:
                    detected = True
                    break
            if detected:
                break
        if detected:
            fp_count += 1
            loc_info = locs[0] if locs else {"file": "?", "start": "?", "end": "?"}
            print(f"  FP: {key} [{cat}] {loc_info['file']}:{loc_info['start']}-{loc_info['end']}")

    if fp_count == 0:
        print("  None! No false positives.")
    print()


def show_summary(test_cases):
    """Show ground truth summary when DB doesn't exist."""
    cats = sorted(set(tc["category"] for tc in test_cases))
    print("%-22s %-6s %-8s %-8s %-8s" % ("Category", "CWE", "Total", "Vuln", "Safe"))
    print("-" * 55)
    total = vuln_total = safe_total = 0
    for cat in cats:
        cat_cases = [tc for tc in test_cases if tc["category"] == cat]
        vuln = sum(1 for tc in cat_cases if tc["vulnerable"])
        safe = sum(1 for tc in cat_cases if not tc["vulnerable"])
        cwe = cat_cases[0].get("cwe", 0)
        total += len(cat_cases)
        vuln_total += vuln
        safe_total += safe
        print("%-22s %-6d %-8d %-8d %-8d" % (cat, cwe, len(cat_cases), vuln, safe))
    print("-" * 55)
    print("%-22s %-6s %-8d %-8d %-8d" % ("TOTAL", "", total, vuln_total, safe_total))


if __name__ == "__main__":
    main()
