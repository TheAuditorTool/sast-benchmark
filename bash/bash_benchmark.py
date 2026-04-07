#!/usr/bin/env python3
"""
Bash SAST Benchmark Scoring Script
====================================
Scores any SAST tool's findings against expectedresults-0.5.1.csv.

Architecture: 1 file = 1 test case.
  - benchmark_test_NNNNN.sh in bash/testcode/ is the test unit.
  - CSV key = file stem (e.g., "benchmark_test_00001")
  - Scoring: tool flags the file for the correct category -> TP or FP.
  - No annotation markers in source files.

Usage:
    python3 bash_benchmark.py

Requires .pf/repo_index.db produced by running a SAST tool on this directory.
Adapt RULE_MAP and SINK_MAP for your tool's rule names.

Scoring metric: TPR - FPR (Youden's J). 0% = random guessing. +100% = perfect.
"""
import sqlite3
import os
from collections import defaultdict
from pathlib import Path

# ============================================================================
# Configuration
# ============================================================================
BENCHMARK_ROOT = Path(os.path.dirname(os.path.abspath(__file__)))
DB_PATH = BENCHMARK_ROOT / ".pf" / "repo_index.db"
GROUND_TRUTH_PATH = BENCHMARK_ROOT / "expectedresults-0.5.2.csv"
TESTCODE_DIR = BENCHMARK_ROOT / "testcode"

# Rule name -> benchmark category mapping
RULE_MAP = {
    # Command injection (CWE-78)
    "bash-eval-injection": ["cmdi", "codeinj"],
    "bash-exec-injection": "cmdi",
    "bash-command-injection-taint": "cmdi",
    "bash-read-without-r": "cmdi",
    "bash-ifs-modified": "cmdi",
    "bash-printf-format-injection": "cmdi",
    "bash-sudo-variable": "cmdi",
    "bash-variable-as-command": "cmdi",
    "bash-variable-command": ["cmdi", "codeinj"],
    "bash-backtick-injection": "cmdi",
    "bash-indirect-expansion": "cmdi",
    "bash-environment-injection": "cmdi",
    "bash-path-modification": "cmdi",
    "bash-arithmetic-injection": "cmdi",
    "bash-unquoted-cmd-injection": "cmdi",
    "bash-xargs-injection": "cmdi",
    "bash-mail-injection": "cmdi",
    # Code injection (CWE-94)
    "bash-source-injection": "codeinj",
    "bash-bash-c-injection": "codeinj",
    "bash-sh-c-injection": "codeinj",
    "bash-zsh-c-injection": "codeinj",
    "bash-ksh-c-injection": "codeinj",
    "bash-trap-injection": "codeinj",
    "bash-json-body-injection": "codeinj",
    # Unquoted expansion (CWE-78 variant)
    "bash-unquoted-expansion": "unquoted",
    "bash-unquoted-dangerous": "unquoted",
    # Hardcoded credentials (CWE-798)
    "bash-hardcoded-credential": "hardcoded_creds",
    "secret-hardcoded-assignment": "hardcoded_creds",
    # Weak crypto (CWE-327)
    "bash-weak-crypto": "weakcrypto",
    # Insecure permissions (CWE-732)
    "bash-chmod-777": "insecure_perms",
    "bash-chmod-666": "insecure_perms",
    "bash-insecure-umask": "insecure_perms",
    # SSL/TLS bypass (CWE-295)
    "bash-ssl-bypass": "ssl_bypass",
    "bash-ssh-hostkey-bypass": "ssl_bypass",
    # Information disclosure (CWE-200/209)
    "bash-debug-mode-leak": "infodisclosure",
    "bash-env-dump": "infodisclosure",
    "bash-sensitive-in-error": "infodisclosure",
    "bash-json-response-injection": "infodisclosure",
    # RCE pipe-to-shell (CWE-94)
    "bash-curl-pipe-bash": "rce",
    # Insecure temp files (CWE-377)
    "bash-unsafe-temp": "insecure_temp",
    # Weak randomness (CWE-330)
    "bash-weak-random": "weakrand",
    # Race condition / TOCTOU (CWE-362)
    "bash-toctou-race": "race_condition",
    # Authentication bypass (CWE-287/306)
    "bash-missing-auth-check": "auth_bypass",
    "bash-env-auth-bypass": "auth_bypass",
    "bash-empty-credential-bypass": "auth_bypass",
    "bash-missing-webhook-signature": "auth_bypass",
    "bash-timing-unsafe-compare": "auth_bypass",
    # Log injection (CWE-117)
    "bash-log-injection": "loginjection",
    # Privilege escalation (CWE-250)
    "bash-privilege-escalation": "privilege_escalation",
    "bash-sudo-user-input": "privilege_escalation",
    # Denial of service (CWE-770)
    "bash-resource-exhaustion": "dos",
    "bash-unbounded-operation": "dos",
    # Cleartext transmission (CWE-319)
    "bash-cleartext-transmission": "cleartext_tx",
    "bash-http-credentials": "cleartext_tx",
    # SSRF
    "ssrf-taint": "ssrf",
    # Path traversal (CWE-22)
    "bash-path-traversal": "pathtraver",
}

NOISE_RULES = {
    "bash-missing-set-e",
    "bash-missing-set-u",
    "bash-relative-sensitive-cmd",
    "deadcode-function",
    "api-missing-auth",
}

SINK_MAP = {
    "Command Injection": "cmdi",
    "SQL Injection": "sqli",
    "Path Traversal": "pathtraver",
    "Server-Side Request Forgery (SSRF)": "ssrf",
    "Information Disclosure": "infodisclosure",
    "Weak Cryptography": "weakcrypto",
    "Remote Code Execution": "rce",
    "Code Injection": "codeinj",
    "Weak Randomness": "weakrand",
    "Race Condition": "race_condition",
    "Authentication Bypass": "auth_bypass",
    "Log Injection": "loginjection",
    "Execution with Unnecessary Privileges": "privilege_escalation",
    "Privilege Escalation": "privilege_escalation",
    "Resource Exhaustion": "dos",
    "Denial of Service": "dos",
    "Cleartext Transmission": "cleartext_tx",
}


# ============================================================================
# CSV Parser
# ============================================================================
def parse_ground_truth(path):
    """Parse expectedresults CSV. Format: test name,category,real vulnerability,CWE"""
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
                "key": parts[0].strip(),           # "benchmark_test_00001"
                "category": parts[1].strip(),
                "vulnerable": parts[2].strip().lower() == "true",
                "cwe": int(parts[3].strip()),
            })
    return test_cases


# ============================================================================
# Scoring
# ============================================================================
def main():
    test_cases = parse_ground_truth(str(GROUND_TRUTH_PATH))
    print(f"Loaded {len(test_cases)} test cases from {GROUND_TRUTH_PATH.name}")

    # Verify source files exist
    missing_files = []
    for tc in test_cases:
        src = TESTCODE_DIR / (tc["key"] + ".sh")
        if not src.exists():
            missing_files.append(tc["key"])
    if missing_files:
        print(f"\nWARNING: {len(missing_files)} test case files not found in testcode/:")
        for k in missing_files[:5]:
            print(f"  testcode/{k}.sh")
        if len(missing_files) > 5:
            print(f"  ... and {len(missing_files) - 5} more")
        print()

    if not DB_PATH.exists():
        print(f"\nERROR: Database not found at {DB_PATH}")
        print("Run your SAST tool on the benchmark directory first.")
        print("\nGround truth summary:\n")
        show_ground_truth_summary(test_cases)
        return

    conn = sqlite3.connect(str(DB_PATH))
    c = conn.cursor()

    # Collect findings: relative_file_path -> set of categories detected
    # File-based scoring: any finding in the file for the right category counts.
    file_categories = defaultdict(set)

    # Track 1: pattern_findings (rule results)
    c.execute("SELECT file, rule FROM pattern_findings")
    for filepath, rule in c.fetchall():
        if rule not in NOISE_RULES:
            cats = RULE_MAP.get(rule)
            if cats:
                if isinstance(cats, str):
                    cats = [cats]
                rel = filepath.replace("\\", "/")
                for cat in cats:
                    file_categories[rel].add(cat)

    # Track 2: resolved_flow_audit (taint-confirmed)
    c.execute(
        "SELECT sink_file, vulnerability_type "
        "FROM resolved_flow_audit WHERE status = 'VULNERABLE'"
    )
    for filepath, vt in c.fetchall():
        cat = SINK_MAP.get(vt)
        if cat:
            rel = filepath.replace("\\", "/")
            file_categories[rel].add(cat)

    conn.close()

    # Score each test case by filename
    cats = sorted(set(tc["category"] for tc in test_cases))
    results = {}

    for cat in cats:
        tp = fp = fn = tn = 0
        cat_cases = [tc for tc in test_cases if tc["category"] == cat]

        for tc in cat_cases:
            # Expected file: testcode/benchmark_test_NNNNN.sh
            rel_path = f"testcode/{tc['key']}.sh"
            detected_cats = file_categories.get(rel_path, set())
            detected = cat in detected_cats

            if tc["vulnerable"]:
                if detected:
                    tp += 1
                else:
                    fn += 1
            else:
                if detected:
                    fp += 1
                else:
                    tn += 1

        results[cat] = {
            "tp": tp, "fp": fp, "fn": fn, "tn": tn,
            "cwe": cat_cases[0]["cwe"] if cat_cases else 0,
        }

    # Print scorecard
    print()
    print(
        "%-20s %-6s %-5s %-5s %-5s %-5s %7s %7s %7s"
        % ("Category", "CWE", "TP", "FP", "FN", "TN", "TPR", "FPR", "Score")
    )
    print("-" * 78)

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
            "%-20s %-6d %-5d %-5d %-5d %-5d %6.1f%% %6.1f%% %+6.1f%%"
            % (cat, cwe, tp, fp, fn, tn, tpr * 100, fpr * 100, score * 100)
        )

    overall_tpr = total_tp / (total_tp + total_fn) if (total_tp + total_fn) else 0
    overall_fpr = total_fp / (total_fp + total_tn) if (total_fp + total_tn) else 0
    overall_score = overall_tpr - overall_fpr

    print("-" * 78)
    print(
        "%-20s %-6s %-5d %-5d %-5d %-5d %6.1f%% %6.1f%% %+6.1f%%"
        % ("OVERALL", "", total_tp, total_fp, total_fn, total_tn,
           overall_tpr * 100, overall_fpr * 100, overall_score * 100)
    )

    # FN analysis
    print("\n\n=== FALSE NEGATIVES (Missed Vulnerabilities) ===\n")
    fn_count = 0
    for tc in test_cases:
        if not tc["vulnerable"]:
            continue
        rel_path = f"testcode/{tc['key']}.sh"
        detected = tc["category"] in file_categories.get(rel_path, set())
        if not detected:
            fn_count += 1
            print(f"  FN: {tc['key']} [{tc['category']}]  testcode/{tc['key']}.sh")
    if fn_count == 0:
        print("  None. All vulnerabilities detected.")

    # FP analysis
    print("\n\n=== FALSE POSITIVES (Incorrect Flags on Safe Code) ===\n")
    fp_count = 0
    for tc in test_cases:
        if tc["vulnerable"]:
            continue
        rel_path = f"testcode/{tc['key']}.sh"
        detected = tc["category"] in file_categories.get(rel_path, set())
        if detected:
            fp_count += 1
            print(f"  FP: {tc['key']} [{tc['category']}]  testcode/{tc['key']}.sh")
    if fp_count == 0:
        print("  None. No false positives.")

    print()


def show_ground_truth_summary(test_cases):
    cats = sorted(set(tc["category"] for tc in test_cases))
    print("%-20s %-6s %-8s %-8s %-8s" % ("Category", "CWE", "Total", "Vuln", "Safe"))
    print("-" * 55)
    total = vuln_total = safe_total = 0
    for cat in cats:
        cat_cases = [tc for tc in test_cases if tc["category"] == cat]
        vuln = sum(1 for tc in cat_cases if tc["vulnerable"])
        safe = sum(1 for tc in cat_cases if not tc["vulnerable"])
        cwe = cat_cases[0]["cwe"]
        total += len(cat_cases)
        vuln_total += vuln
        safe_total += safe
        print("%-20s %-6d %-8d %-8d %-8d" % (cat, cwe, len(cat_cases), vuln, safe))
    print("-" * 55)
    print("%-20s %-6s %-8d %-8d %-8d" % ("TOTAL", "", total, vuln_total, safe_total))
    print(
        f"\nTP/TN split: {vuln_total}/{safe_total} "
        f"({vuln_total * 100 / total:.1f}% / {safe_total * 100 / total:.1f}%)"
    )


if __name__ == "__main__":
    main()
