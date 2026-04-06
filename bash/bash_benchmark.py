#!/usr/bin/env python3
"""
Bash SAST Benchmark Scoring Script
===================================
Scores any SAST tool's findings against expectedresults-0.3.1.csv.

Usage:
    python3 bash_benchmark.py

The script reads findings from a .pf/repo_index.db database (produced by
running a SAST tool on this directory) and compares against the ground truth.
Adapt RULE_MAP and SINK_MAP for your tool's rule names and vulnerability types.

Scoring: TPR - FPR (Youden's J). 0% = random guessing. +100% = perfect.
"""
import sqlite3
import os
import re
from collections import defaultdict
from pathlib import Path

# ============================================================================
# Configuration
# ============================================================================
BENCHMARK_ROOT = Path(os.path.dirname(os.path.abspath(__file__)))
DB_PATH = BENCHMARK_ROOT / ".pf" / "repo_index.db"
GROUND_TRUTH_PATH = BENCHMARK_ROOT / "expectedresults-0.4.0.csv"

# Rule name -> benchmark category mapping
# VERIFIED against actual Desktop/bash/.pf/repo_index.db (2026-03-19)
# 27 distinct rules fire for bash. Each maps to a benchmark category.
RULE_MAP = {
    # Command injection (CWE-78) — 12 rules
    "bash-eval-injection": ["cmdi", "codeinj"],  # eval/bash -c — both CWE-78 and CWE-94
    "bash-exec-injection": "cmdi",              # exec with variable
    "bash-command-injection-taint": "cmdi",     # IFDS-confirmed taint->command flows
    "bash-read-without-r": "cmdi",              # read without -r flag (backslash interp)
    "bash-ifs-modified": "cmdi",                # IFS manipulation without restore
    "bash-printf-format-injection": "cmdi",     # printf with variable format string
    "bash-sudo-variable": "cmdi",               # sudo $cmd
    "bash-variable-as-command": "cmdi",         # $cmd arg1 arg2
    "bash-variable-command": ["cmdi", "codeinj"],  # variable as command — both CWE-78/94
    "bash-backtick-injection": "cmdi",          # `$cmd` backtick substitution
    "bash-indirect-expansion": "cmdi",          # ${!var} indirect expansion
    "bash-environment-injection": "cmdi",       # LD_PRELOAD, LD_LIBRARY_PATH
    "bash-path-modification": "cmdi",           # PATH=./bin:$PATH
    "bash-arithmetic-injection": "cmdi",        # $(( )) arithmetic expansion with variable
    "bash-unquoted-cmd-injection": "cmdi",     # Unquoted expansion in dangerous commands (rm, ssh, git, etc)
    "bash-xargs-injection": "cmdi",            # xargs with -I flag or unsafe pipeline splitting
    "bash-mail-injection": "cmdi",             # mail/sendmail with variable expansion
    # Code injection (CWE-94) — 7 rules
    "bash-source-injection": "codeinj",         # source/dot with variable path
    "bash-bash-c-injection": "codeinj",         # bash -c with variable expansion
    "bash-sh-c-injection": "codeinj",           # sh -c with variable expansion
    "bash-zsh-c-injection": "codeinj",          # zsh -c with variable expansion
    "bash-ksh-c-injection": "codeinj",          # ksh -c with variable expansion
    "bash-trap-injection": "codeinj",           # trap with variable command string
    # Unquoted expansion (CWE-78 variant) — 2 rules
    "bash-unquoted-expansion": "unquoted",      # unquoted vars in commands
    "bash-unquoted-dangerous": "unquoted",      # unquoted + dangerous cmd (rm, cp, mv)
    # Hardcoded credentials (CWE-798) — 2 rules
    "bash-hardcoded-credential": "hardcoded_creds",   # PASSWORD/TOKEN/SECRET vars
    "secret-hardcoded-assignment": "hardcoded_creds",  # language-agnostic secret rule
    # Weak crypto (CWE-327) — 1 rule
    "bash-weak-crypto": "weakcrypto",           # md5sum, sha1sum
    # Insecure permissions (CWE-732) — 3 rules
    "bash-chmod-777": "insecure_perms",         # chmod 777, chmod a+rwx (symbolic)
    "bash-chmod-666": "insecure_perms",         # chmod 666
    "bash-insecure-umask": "insecure_perms",    # umask 000
    # SSL/TLS bypass (CWE-295) — 2 rules
    "bash-ssl-bypass": "ssl_bypass",            # curl -k, wget --no-check-certificate, env var bypass
    "bash-ssh-hostkey-bypass": "ssl_bypass",    # SSH StrictHostKeyChecking=no
    # Information disclosure (CWE-200/209) — 4 rules
    "bash-debug-mode-leak": "infodisclosure",   # set -x / set -o xtrace
    "bash-env-dump": "infodisclosure",             # env/printenv dumps secrets (CWE-200)
    "bash-sensitive-in-error": "infodisclosure",   # echo/printf leaks sensitive vars (CWE-209)
    "bash-json-response-injection": "infodisclosure",  # unescaped var in JSON response (CWE-209)
    # RCE pipe-to-shell (CWE-94) — 1 rule
    "bash-curl-pipe-bash": "rce",               # curl|bash, wget|bash
    # Insecure temp files (CWE-377)
    "bash-unsafe-temp": "insecure_temp",        # predictable /tmp paths
    # Weak randomness (CWE-330) — v0.3.1
    "bash-weak-random": "weakrand",             # $RANDOM for security-sensitive values
    # Race condition / TOCTOU (CWE-362) — v0.3.1
    "bash-toctou-race": "race_condition",       # check-then-use file operations
    # Authentication bypass (CWE-287/306) — v0.3.1
    "bash-missing-auth-check": "auth_bypass",   # functions that skip authentication
    "bash-env-auth-bypass": "auth_bypass",      # SKIP_AUTH env var patterns
    "bash-empty-credential-bypass": "auth_bypass",  # empty cred comparison without -z guard
    "bash-missing-webhook-signature": "auth_bypass", # webhook body parsed without HMAC
    "bash-timing-unsafe-compare": "auth_bypass",    # [[ == ]] on secrets without constant-time
    # JSON body injection (CWE-94) — curl -d with unescaped variable expansion
    "bash-json-body-injection": "codeinj",
    # Log injection (CWE-117) — v0.4.0
    "bash-log-injection": "loginjection",
    # Privilege escalation (CWE-250) — v0.4.0
    "bash-privilege-escalation": "privilege_escalation",
    "bash-sudo-user-input": "privilege_escalation",
    # Denial of service (CWE-770) — v0.4.0
    "bash-resource-exhaustion": "dos",
    "bash-unbounded-operation": "dos",
    # Cleartext transmission (CWE-319) — v0.4.0
    "bash-cleartext-transmission": "cleartext_tx",
    "bash-http-credentials": "cleartext_tx",
    # Cross-category taint rules
    "ssrf-taint": "ssrf",                          # SSRF only — codeinj covered by bash-json-body-injection
}

# Rules to IGNORE in scoring (not security-relevant for benchmark categories)
NOISE_RULES = {
    "bash-missing-set-e",       # Missing errexit — code quality, not security
    "bash-missing-set-u",       # Missing nounset — code quality, not security
    "bash-relative-sensitive-cmd",  # Relative path for chmod/rm — fires 38x, different concern
    "deadcode-function",
    "api-missing-auth",
}

# Taint vulnerability_type -> benchmark category
# VERIFIED against actual resolved_flow_audit.vulnerability_type values
SINK_MAP = {
    "Command Injection": "cmdi",
    "SQL Injection": "sqli",
    "Path Traversal": "pathtraver",
    "Server-Side Request Forgery (SSRF)": "ssrf",  # SSRF only — codeinj covered by structural rule
    "Information Disclosure": "infodisclosure",
    "Weak Cryptography": "weakcrypto",
    "Remote Code Execution": "rce",
    "Code Injection": "codeinj",
    # v0.3.1 additions
    "Weak Randomness": "weakrand",
    "Race Condition": "race_condition",
    "Authentication Bypass": "auth_bypass",
    # v0.4.0 additions
    "Log Injection": "loginjection",
    "Execution with Unnecessary Privileges": "privilege_escalation",
    "Privilege Escalation": "privilege_escalation",
    "Resource Exhaustion": "dos",
    "Denial of Service": "dos",
    "Cleartext Transmission": "cleartext_tx",
}

# ============================================================================
# CSV Parser (OWASP standard format — matches Go/Java/Python benchmarks)
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
                "key": parts[0].strip(),
                "category": parts[1].strip(),
                "vulnerable": parts[2].strip().lower() == "true",
                "cwe": int(parts[3].strip()),
            })

    return test_cases


# ============================================================================
# Source File Scanner (vuln-code-snippet extraction)
# ============================================================================
def scan_annotations(root_dir):
    """Scan source files for vuln-code-snippet start/end markers.
    Returns dict: key -> [{file, start_line, end_line}, ...]
    """
    snippets = {}
    pat_start = re.compile(r"vuln-code-snippet\s+start\s+(.+)")
    pat_end = re.compile(r"vuln-code-snippet\s+end\s+(.+)")

    for dirpath, _, filenames in os.walk(root_dir):
        for fn in filenames:
            if not fn.endswith(".sh"):
                continue

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
    # Parse ground truth
    test_cases = parse_ground_truth(str(GROUND_TRUTH_PATH))
    print(f"Loaded {len(test_cases)} test cases from ground truth")

    # Scan annotations
    snippets = scan_annotations(str(BENCHMARK_ROOT))
    print(f"Found {len(snippets)} annotated snippets in source files")

    # Verify coverage
    missing_annotations = []
    for tc in test_cases:
        if tc["key"] not in snippets:
            missing_annotations.append(tc["key"])
    if missing_annotations:
        print(f"\nWARNING: {len(missing_annotations)} test cases have no source annotation:")
        for k in missing_annotations[:10]:
            print(f"  - {k}")
        if len(missing_annotations) > 10:
            print(f"  ... and {len(missing_annotations) - 10} more")
        print()

    # Load findings from DB
    if not DB_PATH.exists():
        print(f"\nERROR: Database not found at {DB_PATH}")
        print("Run your SAST tool on the benchmark directory first.")
        print("\nShowing ground truth summary instead:\n")
        show_ground_truth_summary(test_cases)
        return

    conn = sqlite3.connect(str(DB_PATH))
    c = conn.cursor()

    # Collect findings: file -> set of (line, category)
    findings = defaultdict(set)

    # Track 1: pattern_findings (rule results)
    c.execute("SELECT file, line, rule FROM pattern_findings")
    for f, ln, r in c.fetchall():
        if r not in NOISE_RULES:
            cats = RULE_MAP.get(r)
            if cats:
                if isinstance(cats, str):
                    cats = [cats]
                for cat in cats:
                    findings[f].add((ln, cat))

    # Track 2: resolved_flow_audit (taint-confirmed)
    c.execute(
        "SELECT sink_file, sink_line, vulnerability_type "
        "FROM resolved_flow_audit WHERE status = 'VULNERABLE'"
    )
    for f, ln, vt in c.fetchall():
        cats = SINK_MAP.get(vt)
        if cats:
            if isinstance(cats, str):
                cats = [cats]
            for cat in cats:
                findings[f].add((ln, cat))

    conn.close()

    # Score each test case
    cats = sorted(set(tc["category"] for tc in test_cases))
    results = {}

    for cat in cats:
        tp = fp = fn = tn = 0
        cat_cases = [tc for tc in test_cases if tc["category"] == cat]

        for tc in cat_cases:
            key = tc["key"]
            is_vulnerable = tc["vulnerable"]
            detected = False

            # Check if any finding falls within annotated line range
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

    # Overall
    overall_tpr = total_tp / (total_tp + total_fn) if (total_tp + total_fn) else 0
    overall_fpr = total_fp / (total_fp + total_tn) if (total_fp + total_tn) else 0
    overall_score = overall_tpr - overall_fpr

    print("-" * 78)
    print(
        "%-20s %-6s %-5d %-5d %-5d %-5d %6.1f%% %6.1f%% %+6.1f%%"
        % (
            "OVERALL",
            "",
            total_tp,
            total_fp,
            total_fn,
            total_tn,
            overall_tpr * 100,
            overall_fpr * 100,
            overall_score * 100,
        )
    )

    # FN analysis
    print("\n\n=== FALSE NEGATIVES (Missed Vulnerabilities) ===\n")
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
            print(
                f"  FN: {key} [{tc['category']}] "
                f"{loc_info['file']}:{loc_info['start']}-{loc_info['end']} "
                f"-- {tc.get('description', '')}"
            )

    if fn_count == 0:
        print("  None! All vulnerabilities detected.")

    # FP analysis
    print("\n\n=== FALSE POSITIVES (Incorrect Flags on Safe Code) ===\n")
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
            print(
                f"  FP: {key} [{tc['category']}] "
                f"{loc_info['file']}:{loc_info['start']}-{loc_info['end']} "
                f"-- {tc.get('description', '')}"
            )

    if fp_count == 0:
        print("  None! No false positives.")

    print()


def show_ground_truth_summary(test_cases):
    """Show summary when DB doesn't exist yet."""
    cats = sorted(set(tc["category"] for tc in test_cases))
    print("%-20s %-6s %-8s %-8s %-8s" % ("Category", "CWE", "Total", "Vuln", "Safe"))
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
        print(
            "%-20s %-6d %-8d %-8d %-8d"
            % (cat, cwe, len(cat_cases), vuln, safe)
        )
    print("-" * 55)
    print(
        "%-20s %-6s %-8d %-8d %-8d"
        % ("TOTAL", "", total, vuln_total, safe_total)
    )
    print(
        f"\nTP/TN split: {vuln_total}/{safe_total} "
        f"({vuln_total*100/total:.1f}% / {safe_total*100/total:.1f}%)"
    )


if __name__ == "__main__":
    main()
