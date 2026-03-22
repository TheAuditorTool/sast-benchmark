#!/usr/bin/env python3
"""Convert TheAuditor's repo_index.db output to standard SARIF 2.1.0.

Usage:
    python convert_theauditor.py <path/to/.pf/repo_index.db> > output.sarif
    python score_sarif.py output.sarif

This bridge script reads TheAuditor's proprietary database tables
(pattern_findings, resolved_flow_audit) and emits a valid SARIF 2.1.0
JSON document. This allows TheAuditor users to use the tool-agnostic
score_sarif.py scorer.

Zero external dependencies -- stdlib only (uses sqlite3 built-in module).
"""

import json
import sqlite3
import sys


RULE_MAP = {
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
}

NOISE_RULES = {"deadcode-function", "api-missing-auth"}


def convert_db_to_sarif(db_path):
    """Read TheAuditor DB and produce SARIF dict."""
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
            key = (file_path, line, category)
            if key not in seen:
                seen.add(key)
                results.append({
                    "ruleId": category,
                    "level": "error",
                    "message": {"text": "Finding from rule: %s" % rule},
                    "locations": [{
                        "physicalLocation": {
                            "artifactLocation": {"uri": file_path},
                            "region": {"startLine": line},
                        }
                    }],
                })
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
            key = (sink_file, sink_line, category)
            if key not in seen:
                seen.add(key)
                results.append({
                    "ruleId": "taint:%s" % category,
                    "level": "error",
                    "message": {"text": "Taint flow: %s" % vuln_type},
                    "locations": [{
                        "physicalLocation": {
                            "artifactLocation": {"uri": sink_file},
                            "region": {"startLine": sink_line},
                        }
                    }],
                })
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
    if len(sys.argv) < 2:
        print(
            "Usage: python convert_theauditor.py <path/to/.pf/repo_index.db>",
            file=sys.stderr,
        )
        print(file=sys.stderr)
        print("Converts TheAuditor's database output to SARIF 2.1.0 JSON.", file=sys.stderr)
        print("Output goes to stdout. Pipe to score_sarif.py:", file=sys.stderr)
        print(file=sys.stderr)
        print(
            "  python convert_theauditor.py .pf/repo_index.db > output.sarif",
            file=sys.stderr,
        )
        print("  python score_sarif.py output.sarif", file=sys.stderr)
        sys.exit(1)

    db_path = sys.argv[1]
    sarif = convert_db_to_sarif(db_path)
    json.dump(sarif, sys.stdout, indent=2)
    print()


if __name__ == "__main__":
    main()
