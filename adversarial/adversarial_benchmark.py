#!/usr/bin/env python3
"""
Adversarial Evasion Benchmark Scoring Script
=============================================
Scores SAST tools against expectedresults-0.1.0.csv for evasion detection.

Usage:
    python3 adversarial_benchmark.py

Reads findings from .pf/repo_index.db (evasion_findings + pattern_findings)
and compares against ground truth.

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

# EIDL signal -> benchmark category mapping
SIGNAL_MAP = {
    "PAYLOAD_CAMOUFLAGE": "unicode_payload",
}

# Pattern rule -> benchmark category mapping
RULE_MAP = {
    # Future rules as they're implemented
    "eidl-bidi-override": "visual_deception",
    "eidl-homoglyph-identifier": "visual_deception",
    "eidl-dynamic-code-build": "dynamic_construction",
    "eidl-install-hook-exec": "supply_chain",
    "eidl-resource-payload": "supply_chain",
    "eidl-prompt-injection": "ai_prompt_injection",
    "eidl-c2-fingerprint": "c2_fingerprint",
}

# Taint vulnerability_type -> benchmark category
SINK_MAP = {
    "Code Injection": "dynamic_construction",
    "Command Injection": "supply_chain",
}

# Rules to ignore
NOISE_RULES = {
    "deadcode-function",
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
    """Scan source files for vuln-code-snippet start/end markers."""
    snippets = {}
    pat_start = re.compile(r"vuln-code-snippet\s+start\s+(.+)")
    pat_end = re.compile(r"vuln-code-snippet\s+end\s+(.+)")

    for dirpath, _, filenames in os.walk(root_dir):
        for fn in filenames:
            if fn.endswith((".py", ".js", ".go", ".rs", ".php", ".sh", ".json", ".txt")):
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
                        for k in m.group(1).strip().split():
                            opens[k.strip()] = i

                    m = pat_end.search(ln)
                    if m:
                        for k in m.group(1).strip().split():
                            k = k.strip()
                            if k in opens:
                                if k not in snippets:
                                    snippets[k] = []
                                snippets[k].append({
                                    "file": rel,
                                    "start": opens.pop(k),
                                    "end": i,
                                })

    return snippets


# ============================================================================
# Scoring
# ============================================================================
def main():
    test_cases = parse_ground_truth(str(GROUND_TRUTH_PATH))
    print(f"Loaded {len(test_cases)} test cases from ground truth")

    snippets = scan_annotations(str(BENCHMARK_ROOT))
    print(f"Found {len(snippets)} annotated snippets in source files")

    # Check coverage
    missing = [tc["key"] for tc in test_cases if tc["key"] not in snippets]
    if missing:
        print(f"\nWARNING: {len(missing)} test cases have no source annotation:")
        for k in missing[:10]:
            print(f"  - {k}")
        if len(missing) > 10:
            print(f"  ... and {len(missing) - 10} more")
        print()

    # Load findings
    if not DB_PATH.exists():
        print(f"\nERROR: Database not found at {DB_PATH}")
        print("Run your SAST tool on the benchmark directory first.")
        print("\nShowing ground truth summary:\n")
        show_summary(test_cases)
        return

    conn = sqlite3.connect(str(DB_PATH))
    c = conn.cursor()
    findings = defaultdict(set)

    # Track 1: evasion_findings (EIDL signals)
    try:
        c.execute("SELECT file, line, confidence FROM evasion_findings")
        for f, ln, conf in c.fetchall():
            # Map EIDL findings to categories via signal columns
            c2 = conn.cursor()
            c2.execute(
                "SELECT has_time_bomb, has_audience_selective, has_fork_gate, "
                "has_star_gate, has_unmotivated_constant, has_architectural_bypass, "
                "has_hyper_narrow_gate, has_state_desync, has_payload_camouflage "
                "FROM evasion_findings WHERE file = ? AND line = ?",
                (f, ln),
            )
            row = c2.fetchone()
            if row and row[8]:  # has_payload_camouflage
                findings[f].add((ln, "unicode_payload"))
    except sqlite3.OperationalError:
        pass  # Table doesn't exist yet

    # Track 2: pattern_findings
    try:
        c.execute("SELECT file, line, rule FROM pattern_findings")
        for f, ln, r in c.fetchall():
            if r not in NOISE_RULES:
                cat = RULE_MAP.get(r)
                if cat:
                    findings[f].add((ln, cat))
    except sqlite3.OperationalError:
        pass

    # Track 3: resolved_flow_audit
    try:
        c.execute(
            "SELECT sink_file, sink_line, vulnerability_type "
            "FROM resolved_flow_audit WHERE status = 'VULNERABLE'"
        )
        for f, ln, vt in c.fetchall():
            cat = SINK_MAP.get(vt)
            if cat:
                findings[f].add((ln, cat))
    except sqlite3.OperationalError:
        pass

    conn.close()

    # Score
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
    print("\n\n=== FALSE NEGATIVES (Missed Evasion Patterns) ===\n")
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
        print("  None! All evasion patterns detected.")

    # FP analysis
    print("\n\n=== FALSE POSITIVES (Safe Code Incorrectly Flagged) ===\n")
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
    print(f"\nTP/TN split: {vuln_total}/{safe_total} ({vuln_total*100/total:.1f}% / {safe_total*100/total:.1f}%)")


if __name__ == "__main__":
    main()
