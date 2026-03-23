#!/usr/bin/env python3
"""Convert TheAuditor's repo_index.db output to standard SARIF 2.1.0.

Usage:
    python convert_theauditor.py <db_path> --language go
    python convert_theauditor.py <db_path> --language bash --benchmark-dir ../bash
    python convert_theauditor.py <db_path> --language rust --benchmark-dir ../rust
    python convert_theauditor.py <db_path>   # auto-detect from rule prefixes

Then score:
    python score_sarif.py output.sarif expectedresults.csv

For Go: findings match test cases by filename (benchmark_test_NNNNN.go).
For Bash/Rust: findings are resolved to test case keys via vuln-code-snippet
annotations, then embedded in SARIF properties.testCaseKey for the scorer.

Zero external dependencies -- stdlib only.
"""

import json
import os
import re
import sqlite3
import sys
from collections import defaultdict

# ============================================================================
# Rule Maps (all languages combined — prefixes prevent collisions)
# ============================================================================

RULE_MAP = {
    # --- Go rules ---
    "go-sql-injection-format": "sqli",
    "go-sql-injection-concat": "sqli",
    "go-command-injection": "cmdi",
    "go-path-traversal": "pathtraver",
    "go-xss-template-bypass": "xss",
    "go-weak-random": "weakrand",
    "go-weak-hash": "weakhash",
    "go-weak-cipher": "weakcipher",
    "go-insecure-cookie": "securecookie",
    "go-insecure-tls": "tlsverify",
    "go-ldap-injection": "ldapi",
    "go-nosql-injection": "nosql",
    "go-log-injection": "loginjection",
    "go-open-redirect": "redirect",
    "go-ssrf": "ssrf",
    "go-deserialization": "deserial",
    "go-trust-boundary": "trustbound",
    "go-hardcoded-credential": "hardcodedcreds",
    "go-jwt-none-algorithm": "authnfailure",
    "go-missing-auth": "authnfailure",
    "go-missing-authz": "authzfailure",
    "go-csrf-missing": "csrf",
    "go-template-injection": "codeinj",
    # --- Bash rules ---
    "bash-eval-injection": "cmdi",
    "bash-exec-injection": "cmdi",
    "bash-command-injection-taint": "cmdi",
    "bash-read-without-r": "cmdi",
    "bash-ifs-modified": "cmdi",
    "bash-printf-format-injection": "cmdi",
    "bash-sudo-variable": "cmdi",
    "bash-variable-as-command": "cmdi",
    "bash-variable-command": "cmdi",
    "bash-backtick-injection": "cmdi",
    "bash-indirect-expansion": "cmdi",
    "bash-environment-injection": "cmdi",
    "bash-path-modification": "cmdi",
    "bash-source-injection": "codeinj",
    "bash-unquoted-expansion": "unquoted",
    "bash-unquoted-dangerous": "unquoted",
    "bash-hardcoded-credential": "hardcoded_creds",
    "secret-hardcoded-assignment": "hardcoded_creds",
    "bash-weak-crypto": "weakcrypto",
    "bash-chmod-777": "insecure_perms",
    "bash-chmod-666": "insecure_perms",
    "bash-ssl-bypass": "ssl_bypass",
    "bash-debug-mode-leak": "infodisclosure",
    "bash-curl-pipe-bash": "rce",
    "bash-unsafe-temp": "insecure_temp",
    "bash-weak-random": "weakrand",
    "bash-toctou-race": "race_condition",
    "bash-missing-auth-check": "auth_bypass",
    "bash-env-auth-bypass": "auth_bypass",
    # --- Rust rules ---
    # Weak random (crypto_analyze.py)
    "rust-weak-random": "weakrand",
    # Taint injection (rust_injection_analyze.py)
    "rust-command-injection-taint": "cmdi",
    "rust-sql-injection-taint": "sqli",
    "rust-path-traversal-taint": "pathtraver",
    "rust-path-traversal-sink": "pathtraver",
    "rust-ssrf-taint": "ssrf",
    "rust-ssrf-sink": "ssrf",
    "express-ssrf-taint": "ssrf",
    # Structural injection (rust_injection_analyze.py)
    "rust-command-injection": "cmdi",
    "rust-sql-injection-format": "sqli",
    "rust-sql-injection-sink": "sqli",
    "rust-sql-injection-structural": "sqli",
    # Polyglot taint rules
    "path-traversal-taint": "pathtraver",
    "ssrf-taint": "ssrf",
    # Memory safety (memory_safety.py + unsafe_analysis.py + ffi_boundary.py)
    "rust-dangerous-import": "memsafety",
    "rust-unsafe-no-safety-comment": "memsafety",
    "rust-unsafe-in-public-api": "memsafety",
    "rust-unsafe-trait-impl": "memsafety",
    "rust-unsafe-public-fn": "memsafety",
    "rust-ffi-variadic": "memsafety",
    "rust-ffi-raw-pointer-param": "memsafety",
    "rust-ffi-raw-pointer-return": "memsafety",
    "rust-ffi-extern-block": "memsafety",
    "rust-ffi-panic-across-boundary": "memsafety",
    # Panic paths (panic_paths.py)
    "rust-panic-unwrap": "memsafety",
    "rust-panic-expect": "memsafety",
    "rust-panic-in-production": "memsafety",
    "rust-todo-in-production": "memsafety",
    "rust-unimplemented-in-production": "memsafety",
    "rust-unreachable-in-production": "memsafety",
    # Integer safety (integer_safety.py)
    "rust-integer-high-risk-function": "intoverflow",
    "rust-truncating-cast": "intoverflow",
    "rust-wrapping-arithmetic-used": "intoverflow",
    # Supply chain + crypto (supply_chain.py + crypto_weakness.py)
    "rust-weak-crypto-dependency": "crypto",
    "rust-deprecated-dependency": "crypto",
    "rust-weak-crypto-call": "crypto",
    "rust-weak-crypto-macro": "crypto",
    "rust-weak-hash-output": "crypto",
    "rust-jwt-algorithm-none": "crypto",
    # Hardcoded secrets
    "hardcoded-credential": "infodisclosure",
    "hardcoded-secret": "infodisclosure",
    # Info disclosure (info_disclosure_analyze.py)
    "rust-error-details-exposed": "infodisclosure",
    "rust-env-vars-dump": "infodisclosure",
    "rust-sensitive-config-exposed": "infodisclosure",
    "rust-hardcoded-secret-fallback": "infodisclosure",
    "rust-sql-in-error-response": "infodisclosure",
    "rust-sensitive-data-logged": "infodisclosure",
    "rust-hardcoded-api-key": "infodisclosure",
    # XSS (xss_analyze.py)
    "xss-taint": "xss",
    "xss-sink": "xss",
    "xss-postmessage-origin": "xss",
    # Deserialization (rust_insecure_deserialization.py)
    "rust-insecure-deserialization": "deser",
    # Input validation (input_validation_analyze.py)
    "rust-missing-input-validation": "inputval",
    # ReDoS (redos_analyze.py)
    "redos-taint": "redos",
    "redos-dynamic-regex": "redos",
    # --- PHP rules ---
    "php-sql-injection": "sqli",
    "php-sql-injection-concat": "sqli",
    "php-sql-injection-taint": "sqli",
    "php-command-injection": "cmdi",
    "php-command-injection-taint": "cmdi",
    "php-code-injection": "codeinj",
    "php-file-inclusion": "fileinclusion",
    "php-file-inclusion-taint": "fileinclusion",
    "php-path-traversal": "pathtraver",
    "php-path-traversal-taint": "pathtraver",
    "php-xss": "xss",
    "php-xss-taint": "xss",
    "php-ssrf": "ssrf",
    "php-ssrf-taint": "ssrf",
    "php-deserialization": "deserial",
    "php-insecure-deserialization": "deserial",
    "php-xxe": "xxe",
    "php-type-juggling": "typejuggling",
    "php-extract-injection": "extract",
    "php-variable-variables": "variablevars",
    "php-unsafe-reflection": "unsafereflect",
    "php-file-upload": "fileupload",
    "php-open-redirect": "redirect",
    "php-weak-hash": "weakhash",
    "php-weak-random": "weakrand",
    "php-weak-cipher": "weakcipher",
    "php-hardcoded-credential": "hardcodedcreds",
    "php-csrf-missing": "csrf",
    "php-header-injection": "headerinj",
    "php-ldap-injection": "ldapi",
    "php-insecure-cookie": "securecookie",
    "php-mass-assignment": "massassign",
    "php-template-injection": "ssti",
}

SINK_MAP = {
    "SQL Injection": "sqli",
    "Command Injection": "cmdi",
    "Path Traversal": "pathtraver",
    "Cross-Site Scripting (XSS)": "xss",
    "Server-Side Request Forgery (SSRF)": "ssrf",
    "NoSQL Injection": "nosql",
    "LDAP Injection": "ldapi",
    "Open Redirect": "redirect",
    "Log Injection": "loginjection",
    "Deserialization": "deserial",
    "Template Injection": "codeinj",
    "Information Disclosure": "infodisclosure",
    "Weak Cryptography": "weakcrypto",
    "Remote Code Execution": "rce",
    "Code Injection": "codeinj",
    "Weak Randomness": "weakrand",
    "Race Condition": "race_condition",
    "Authentication Bypass": "auth_bypass",
    # Rust-specific sink types
    "Cross-Site Scripting": "xss",
    "Server-Side Request Forgery": "ssrf",
    "SSRF": "ssrf",
    "Memory Safety": "memsafety",
    "Insecure Deserialization": "deser",
    "ReDoS": "redos",
    # PHP-specific sink types
    "File Inclusion": "fileinclusion",
    "Type Juggling": "typejuggling",
    "Variable Extraction": "extract",
    "Variable Variables": "variablevars",
    "Unsafe Reflection": "unsafereflect",
    "Unrestricted Upload": "fileupload",
    "Header Injection": "headerinj",
    "Mass Assignment": "massassign",
    "Server-Side Template Injection": "ssti",
    "XXE": "xxe",
}

NOISE_RULES = {
    "deadcode-function",
    "deadcode-module",
    "unused_dependencies",
    "api-missing-auth",
    "bash-missing-set-e",
    "bash-missing-set-u",
    "bash-relative-sensitive-cmd",
}


# ============================================================================
# Annotation Scanner (for Bash/Rust — resolves file+line to test case key)
# ============================================================================

PAT_START = re.compile(r"vuln-code-snippet\s+start\s+(\S+)")
PAT_END = re.compile(r"vuln-code-snippet\s+end\s+(\S+)")


def scan_annotations(benchmark_dir, extensions=(".sh",)):
    """Scan source files for vuln-code-snippet annotations.
    Returns dict: {relative_file_path: [(start_line, end_line, key), ...]}
    """
    file_ranges = defaultdict(list)
    open_snippets = {}

    scan_dirs = [
        os.path.join(benchmark_dir, "apps"),
        os.path.join(benchmark_dir, "testcode"),
    ]

    for scan_dir in scan_dirs:
        if not os.path.isdir(scan_dir):
            continue
        for root, dirs, files in os.walk(scan_dir):
            dirs[:] = [d for d in dirs if d not in
                       (".git", "node_modules", ".auditor_venv", ".pf", "target")]
            for fn in sorted(files):
                if not any(fn.endswith(ext) for ext in extensions):
                    continue
                filepath = os.path.join(root, fn)
                rel = os.path.relpath(filepath, benchmark_dir).replace("\\", "/")
                try:
                    with open(filepath, "r", encoding="utf-8", errors="replace") as f:
                        lines = f.readlines()
                except OSError:
                    continue

                for i, line in enumerate(lines, 1):
                    m = PAT_START.search(line)
                    if m:
                        open_snippets[m.group(1)] = (rel, i)

                    m = PAT_END.search(line)
                    if m:
                        key = m.group(1)
                        if key in open_snippets:
                            start_file, start_line = open_snippets.pop(key)
                            file_ranges[start_file].append(
                                (start_line, i, key)
                            )

    return dict(file_ranges)


def resolve_finding_to_key(file_path, line, file_ranges):
    """Given a finding (file, line), find which test case key it falls within."""
    ranges = file_ranges.get(file_path, [])
    for start, end, key in ranges:
        if start <= line <= end:
            return key
    return None


# ============================================================================
# Converter
# ============================================================================

def detect_language(db_path):
    """Auto-detect language from rule prefixes in the database."""
    conn = sqlite3.connect(db_path)
    c = conn.cursor()
    try:
        c.execute("SELECT DISTINCT rule FROM pattern_findings LIMIT 100")
        rules = [row[0] for row in c.fetchall()]
    except sqlite3.OperationalError:
        rules = []
    conn.close()

    go_count = sum(1 for r in rules if r.startswith("go-"))
    bash_count = sum(1 for r in rules if r.startswith("bash-"))
    rust_count = sum(1 for r in rules if r.startswith("rust-"))
    php_count = sum(1 for r in rules if r.startswith("php-"))

    counts = {"go": go_count, "bash": bash_count, "rust": rust_count, "php": php_count}
    return max(counts, key=counts.get)


def convert_db_to_sarif(db_path, language=None, benchmark_dir=None):
    """Read TheAuditor DB and produce SARIF dict."""
    if language is None:
        language = detect_language(db_path)

    # For annotation-based languages, scan source files
    file_ranges = {}
    if language in ("bash", "rust", "php") and benchmark_dir:
        ext = {"bash": ".sh", "rust": ".rs", "php": ".php"}[language]
        file_ranges = scan_annotations(benchmark_dir, extensions=(ext,))

    conn = sqlite3.connect(db_path)
    c = conn.cursor()

    results = []
    seen = set()

    # Pattern findings
    try:
        c.execute("SELECT file, line, rule FROM pattern_findings")
        for file_path, line, rule in c.fetchall():
            if rule in NOISE_RULES:
                continue
            category = RULE_MAP.get(rule, rule)
            dedup_key = (file_path, line, category)
            if dedup_key in seen:
                continue
            seen.add(dedup_key)

            result = {
                "ruleId": category,
                "level": "error",
                "message": {"text": "Finding from rule: %s" % rule},
                "locations": [{
                    "physicalLocation": {
                        "artifactLocation": {"uri": file_path},
                        "region": {"startLine": line},
                    }
                }],
            }

            # For annotation-based languages, resolve to test case key
            if file_ranges:
                tc_key = resolve_finding_to_key(file_path, line, file_ranges)
                if tc_key:
                    result["properties"] = {"testCaseKey": tc_key}

            results.append(result)
    except sqlite3.OperationalError:
        pass

    # Taint flow findings
    try:
        c.execute(
            "SELECT sink_file, sink_line, vulnerability_type "
            "FROM resolved_flow_audit WHERE status = 'VULNERABLE'"
        )
        for sink_file, sink_line, vuln_type in c.fetchall():
            category = SINK_MAP.get(vuln_type, vuln_type)
            dedup_key = (sink_file, sink_line, category)
            if dedup_key in seen:
                continue
            seen.add(dedup_key)

            result = {
                "ruleId": "taint:%s" % category,
                "level": "error",
                "message": {"text": "Taint flow: %s" % vuln_type},
                "locations": [{
                    "physicalLocation": {
                        "artifactLocation": {"uri": sink_file},
                        "region": {"startLine": sink_line},
                    }
                }],
            }

            if file_ranges:
                tc_key = resolve_finding_to_key(sink_file, sink_line, file_ranges)
                if tc_key:
                    result["properties"] = {"testCaseKey": tc_key}

            results.append(result)
    except sqlite3.OperationalError:
        pass

    conn.close()

    sarif = {
        "$schema": "https://schemastore.azurewebsites.net/schemas/json/sarif-2.1.0-rtm.5.json",
        "version": "2.1.0",
        "runs": [{
            "tool": {
                "driver": {
                    "name": "TheAuditor",
                    "informationUri": "https://github.com/theauditor",
                    "rules": [],
                }
            },
            "results": results,
        }],
    }

    return sarif


def main():
    if len(sys.argv) < 2 or sys.argv[1] in ("-h", "--help"):
        print("Usage: python convert_theauditor.py <db_path> [options]", file=sys.stderr)
        print(file=sys.stderr)
        print("Options:", file=sys.stderr)
        print("  --language go|bash|rust|php  Language (auto-detected if omitted)", file=sys.stderr)
        print("  --benchmark-dir <path>    Benchmark directory (required for bash/rust)", file=sys.stderr)
        print(file=sys.stderr)
        print("Examples:", file=sys.stderr)
        print("  python convert_theauditor.py go/.pf/repo_index.db > go.sarif", file=sys.stderr)
        print("  python convert_theauditor.py bash/.pf/repo_index.db --language bash --benchmark-dir bash > bash.sarif", file=sys.stderr)
        print("  python score_sarif.py bash.sarif bash/expectedresults-0.3.1.csv", file=sys.stderr)
        sys.exit(1)

    db_path = sys.argv[1]
    language = None
    benchmark_dir = None

    i = 2
    while i < len(sys.argv):
        if sys.argv[i] == "--language" and i + 1 < len(sys.argv):
            language = sys.argv[i + 1]
            i += 2
        elif sys.argv[i] == "--benchmark-dir" and i + 1 < len(sys.argv):
            benchmark_dir = sys.argv[i + 1]
            i += 2
        else:
            i += 1

    # Auto-detect benchmark dir if not provided
    if benchmark_dir is None and language in ("bash", "rust", "php"):
        parent = os.path.dirname(os.path.dirname(os.path.abspath(db_path)))
        if os.path.isdir(os.path.join(parent, "testcode")):
            benchmark_dir = parent

    sarif = convert_db_to_sarif(db_path, language=language, benchmark_dir=benchmark_dir)
    json.dump(sarif, sys.stdout, indent=2)
    print()


if __name__ == "__main__":
    main()
