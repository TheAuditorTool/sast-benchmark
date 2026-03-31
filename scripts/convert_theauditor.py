#!/usr/bin/env python3
"""Convert TheAuditor's repo_index.db output to standard SARIF 2.1.0.

Usage:
    python convert_theauditor.py <db_path> --language go
    python convert_theauditor.py <db_path> --language bash --benchmark-dir ../bash
    python convert_theauditor.py <db_path> --language rust --benchmark-dir ../rust
    python convert_theauditor.py <db_path> --language python
    python convert_theauditor.py <db_path>   # auto-detect from rule prefixes

Then score:
    python score_sarif.py output.sarif expectedresults.csv

Matching strategy: CWE-based. SARIF ruleId is the CWE number (e.g., "89").
The scorer matches this against the CSV CWE column. No hand-maintained
RULE_MAP needed -- CWE comes directly from pattern_findings.cwe column
and from VULN_TYPE_TO_CWE for taint flows.

For Go/Python/Java: findings match test cases by filename.
For Bash/Rust/PHP: findings are resolved to test case keys via
vuln-code-snippet annotations.

Zero external dependencies -- stdlib only.
"""

import json
import os
import re
import sqlite3
import sys
from collections import defaultdict

# ============================================================================
# CWE Mapping for taint flows (industry-standard, ~25 entries)
#
# resolved_flow_audit stores vulnerability_type as human-readable strings.
# This maps them to CWE numbers. Stable -- CWE IDs don't change.
# ============================================================================

VULN_TYPE_TO_CWE = {
    "SQL Injection": 89,
    "Command Injection": 78,
    "Path Traversal": 22,
    "Cross-Site Scripting (XSS)": 79,
    "Cross-Site Scripting": 79,
    "Server-Side Request Forgery (SSRF)": 918,
    "Server-Side Request Forgery": 918,
    "SSRF": 918,
    "NoSQL Injection": 943,
    "LDAP Injection": 90,
    "Open Redirect": 601,
    "Log Injection": 117,
    "Insecure Deserialization": 502,
    "Deserialization": 502,
    "Code Injection": 94,
    "Template Injection": 1336,
    "Server-Side Template Injection (SSTI)": 1336,
    "Server-Side Template Injection": 1336,
    "XPath Injection": 643,
    "XML External Entity (XXE)": 611,
    "XXE": 611,
    "Trust Boundary Violation": 501,
    "Weak Cryptography": 327,
    "Weak Randomness": 330,
    "Race Condition": 362,
    "Memory Safety": 119,
    "ReDoS": 1333,
    "Information Disclosure": 200,
    "Data Exposure": 200,
    "Unvalidated Input": 20,
    "Authentication Bypass": 287,
    "Remote Code Execution": 94,
    "Prototype Pollution": 1321,
    "HTTP Header Injection": 113,
    "File Inclusion": 98,
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


def _parse_cwe_number(cwe_str):
    """Extract integer CWE number from 'CWE-89' format. Returns None if invalid."""
    if not cwe_str:
        return None
    m = re.match(r"CWE-(\d+)", cwe_str)
    return int(m.group(1)) if m else None


# ============================================================================
# Annotation Scanner (for Bash/Rust/PHP — resolves file+line to test case key)
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
    python_count = sum(1 for r in rules if r.startswith("python-") or r.startswith("flask-"))
    ruby_count = sum(1 for r in rules if r.startswith("ruby-"))

    counts = {"go": go_count, "bash": bash_count, "rust": rust_count,
              "php": php_count, "python": python_count, "ruby": ruby_count}
    return max(counts, key=counts.get)


def convert_db_to_sarif(db_path, language=None, benchmark_dir=None):
    """Read TheAuditor DB and produce SARIF dict.

    Uses CWE numbers as ruleId instead of category strings.
    CWE comes from pattern_findings.cwe column (already populated by rules)
    and from VULN_TYPE_TO_CWE for resolved_flow_audit taint flows.
    """
    if language is None:
        language = detect_language(db_path)

    # For annotation-based languages, scan source files
    file_ranges = {}
    if language in ("bash", "rust", "php", "ruby") and benchmark_dir:
        ext = {"bash": ".sh", "rust": ".rs", "php": ".php", "ruby": ".rb"}[language]
        file_ranges = scan_annotations(benchmark_dir, extensions=(ext,))

    conn = sqlite3.connect(db_path)
    c = conn.cursor()

    results = []
    seen = set()
    cwe_covered_by_rules = set()  # CWEs already covered by pattern rules

    # Pattern findings -- use CWE from DB directly
    try:
        c.execute("SELECT file, line, rule, cwe FROM pattern_findings")
        for file_path, line, rule, cwe_str in c.fetchall():
            if rule in NOISE_RULES:
                continue
            cwe_num = _parse_cwe_number(cwe_str)
            if cwe_num is None:
                continue  # No CWE = not a scoreable finding

            if rule.endswith("-taint") or rule.endswith("-sink"):
                cwe_covered_by_rules.add(cwe_num)

            dedup_key = (file_path, line, cwe_num)
            if dedup_key in seen:
                continue
            seen.add(dedup_key)

            result = {
                "ruleId": str(cwe_num),
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
            "SELECT sink_file, sink_line, source_file, source_line, vulnerability_type "
            "FROM resolved_flow_audit WHERE status = 'VULNERABLE'"
        )
        for sink_file, sink_line, source_file, source_line, vuln_type in c.fetchall():
            cwe_num = VULN_TYPE_TO_CWE.get(vuln_type)
            if cwe_num is None:
                continue  # Unknown vuln type -- skip rather than emit garbage

            # Cross-file VULNERABLE flows: also emit at source for scoring attribution
            is_cross_file = source_file and source_file != sink_file
            if is_cross_file:
                src_dedup = (source_file, source_line, cwe_num)
                if src_dedup not in seen:
                    seen.add(src_dedup)
                    src_result = {
                        "ruleId": str(cwe_num),
                        "level": "error",
                        "message": {"text": "Taint flow: %s (cross-file)" % vuln_type},
                        "locations": [{
                            "physicalLocation": {
                                "artifactLocation": {"uri": source_file},
                                "region": {"startLine": source_line or 1},
                            }
                        }],
                    }
                    if file_ranges:
                        tc_key = resolve_finding_to_key(source_file, source_line or 1, file_ranges)
                        if tc_key:
                            src_result["properties"] = {"testCaseKey": tc_key}
                    results.append(src_result)

            # Skip if pattern rules already cover this CWE
            if cwe_num in cwe_covered_by_rules:
                continue
            dedup_key = (sink_file, sink_line, cwe_num)
            if dedup_key in seen:
                continue
            seen.add(dedup_key)

            result = {
                "ruleId": "taint:%d" % cwe_num,
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
        print("  --language go|bash|rust|php|python  Language (auto-detected if omitted)", file=sys.stderr)
        print("  --benchmark-dir <path>    Benchmark directory (required for bash/rust/php)", file=sys.stderr)
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
    if benchmark_dir is None and language in ("bash", "rust", "php", "ruby"):
        parent = os.path.dirname(os.path.dirname(os.path.abspath(db_path)))
        if os.path.isdir(os.path.join(parent, "testcode")):
            benchmark_dir = parent

    sarif = convert_db_to_sarif(db_path, language=language, benchmark_dir=benchmark_dir)
    json.dump(sarif, sys.stdout, indent=2)
    print()


if __name__ == "__main__":
    main()
