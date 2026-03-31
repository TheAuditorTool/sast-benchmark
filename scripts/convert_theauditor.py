#!/usr/bin/env python3
"""Convert TheAuditor's repo_index.db output to standard SARIF 2.1.0.

Usage:
    python convert_theauditor.py <db_path> --language php
    python convert_theauditor.py <db_path>                  # auto-detect language
    python convert_theauditor.py <db_path> --force          # regenerate even if current
    python convert_theauditor.py <db_path> --stdout         # legacy: print to stdout

Output: writes <benchmark_dir>/theauditor.sarif with integrity metadata.
If the SARIF already exists and DB+CSV haven't changed, the converter
skips regeneration (hash check). This eliminates stale SARIF divergence
across teams.

Matching strategy: CWE-based. SARIF ruleId is the CWE number (e.g., "89").
The scorer matches this against the CSV CWE column. No hand-maintained
RULE_MAP needed -- CWE comes directly from pattern_findings.cwe column
and from VULN_TYPE_TO_CWE for taint flows.

For Go/Python/Java: findings match test cases by filename.
For Bash/Rust/PHP/Ruby: findings are resolved to test case keys via
vuln-code-snippet annotations.

Zero external dependencies -- stdlib only.
"""

import glob
import hashlib
import json
import os
import re
import sqlite3
import sys
from collections import defaultdict
from datetime import datetime, timezone

CONVERTER_VERSION = "2.0"

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
    "Argument Injection": 88,
    "CRLF Injection": 93,
    "Integer Overflow": 190,
    "Error Message Information Exposure": 209,
    "Improper Certificate Validation": 295,
    "Missing Authentication": 306,
    "Weak Hash": 328,
    "Improper Cryptographic Signature Verification": 347,
    "Cross-Site Request Forgery (CSRF)": 352,
    "Cross-Site Request Forgery": 352,
    "TOCTOU Race Condition": 367,
    "Insecure Temporary File": 377,
    "Unrestricted File Upload": 434,
    "Unsafe Reflection": 470,
    "Log Information Disclosure": 532,
    "Sensitive Cookie Without Secure Flag": 614,
    "Variable Extraction": 621,
    "Dynamic Variable Evaluation": 627,
    "Incorrect Comparison": 697,
    "Incorrect Permission Assignment": 732,
    "Hard-Coded Credentials": 798,
    "Hardcoded Credentials": 798,
    "Missing Authorization": 862,
    "Mass Assignment": 915,
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
# Integrity -- SHA256 hashing for staleness detection
# ============================================================================

def _sha256_file(path):
    """Compute SHA256 hex digest of a file."""
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(65536), b""):
            h.update(chunk)
    return h.hexdigest()


def _find_csv(benchmark_dir):
    """Find the expectedresults-*.csv file in a benchmark directory."""
    pattern = os.path.join(benchmark_dir, "expectedresults-*.csv")
    matches = glob.glob(pattern)
    if len(matches) == 1:
        return matches[0]
    if len(matches) > 1:
        return sorted(matches)[-1]
    return None


def _read_existing_integrity(sarif_path):
    """Read integrity block from an existing SARIF file. Returns dict or None."""
    if not os.path.isfile(sarif_path):
        return None
    try:
        with open(sarif_path, "r", encoding="utf-8") as f:
            data = json.load(f)
        return data.get("runs", [{}])[0].get("properties", {}).get("integrity")
    except (json.JSONDecodeError, KeyError, IndexError):
        return None


def check_staleness(sarif_path, db_path, csv_path):
    """Check if existing SARIF is current.

    Returns:
        (is_current, reason) -- True if SARIF matches DB+CSV, else False + reason string
    """
    integrity = _read_existing_integrity(sarif_path)
    if integrity is None:
        return False, "no integrity metadata (legacy or non-TheAuditor SARIF)"

    current_db_hash = _sha256_file(db_path)
    current_csv_hash = _sha256_file(csv_path)

    if integrity.get("db_sha256") != current_db_hash:
        return False, "DB changed (re-indexed since last conversion)"
    if integrity.get("csv_sha256") != current_csv_hash:
        return False, "CSV changed (ground truth updated since last conversion)"

    return True, "DB+CSV unchanged"


# ============================================================================
# Helpers
# ============================================================================

def _parse_cwe_number(cwe_str):
    """Extract integer CWE number from 'CWE-89' format. Returns None if invalid."""
    if not cwe_str:
        return None
    m = re.match(r"CWE-(\d+)", cwe_str)
    return int(m.group(1)) if m else None


# ============================================================================
# Annotation Scanner (for Bash/Rust/PHP/Ruby -- resolves file+line to test case key)
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


def convert_db_to_sarif(db_path, language=None, benchmark_dir=None, csv_path=None):
    """Read TheAuditor DB and produce SARIF dict with integrity metadata.

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
    cwe_covered_by_rules = set()

    # Pattern findings -- use CWE from DB directly
    try:
        c.execute("SELECT file, line, rule, cwe FROM pattern_findings")
        for file_path, line, rule, cwe_str in c.fetchall():
            if rule in NOISE_RULES:
                continue
            cwe_num = _parse_cwe_number(cwe_str)
            if cwe_num is None:
                continue

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
                continue

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

    # Build integrity block
    integrity = {
        "converter_version": CONVERTER_VERSION,
        "generated_at": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "db_sha256": _sha256_file(db_path),
        "db_path": os.path.basename(db_path),
    }
    if csv_path and os.path.isfile(csv_path):
        integrity["csv_sha256"] = _sha256_file(csv_path)
        integrity["csv_path"] = os.path.basename(csv_path)

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
            "properties": {
                "integrity": integrity,
            },
        }],
    }

    return sarif


def main():
    if len(sys.argv) < 2 or sys.argv[1] in ("-h", "--help"):
        print("Usage: python convert_theauditor.py <db_path> [options]", file=sys.stderr)
        print(file=sys.stderr)
        print("Options:", file=sys.stderr)
        print("  --language go|bash|rust|php|ruby|python  Language (auto-detected if omitted)", file=sys.stderr)
        print("  --benchmark-dir <path>  Benchmark directory (auto-detected from db_path)", file=sys.stderr)
        print("  --force                 Regenerate even if SARIF is current", file=sys.stderr)
        print("  --stdout                Print to stdout instead of writing file (legacy)", file=sys.stderr)
        print(file=sys.stderr)
        print("Output: writes <benchmark_dir>/theauditor.sarif with integrity hashes.", file=sys.stderr)
        print("If DB+CSV haven't changed since last run, skips regeneration.", file=sys.stderr)
        print(file=sys.stderr)
        print("Examples:", file=sys.stderr)
        print("  cd php/ && python ../scripts/convert_theauditor.py .pf/repo_index.db", file=sys.stderr)
        print("  python scripts/convert_theauditor.py php/.pf/repo_index.db --language php", file=sys.stderr)
        sys.exit(1)

    db_path = sys.argv[1]
    language = None
    benchmark_dir = None
    force = False
    use_stdout = False

    i = 2
    while i < len(sys.argv):
        if sys.argv[i] == "--language" and i + 1 < len(sys.argv):
            language = sys.argv[i + 1]
            i += 2
        elif sys.argv[i] == "--benchmark-dir" and i + 1 < len(sys.argv):
            benchmark_dir = sys.argv[i + 1]
            i += 2
        elif sys.argv[i] == "--force":
            force = True
            i += 1
        elif sys.argv[i] == "--stdout":
            use_stdout = True
            i += 1
        else:
            i += 1

    # Auto-detect benchmark dir from db_path (db is at <benchmark>/.pf/repo_index.db)
    if benchmark_dir is None:
        parent = os.path.dirname(os.path.dirname(os.path.abspath(db_path)))
        if os.path.isdir(os.path.join(parent, "testcode")):
            benchmark_dir = parent
        else:
            benchmark_dir = os.getcwd()

    # Auto-detect CSV
    csv_path = _find_csv(benchmark_dir)

    # Canonical output path
    sarif_path = os.path.join(benchmark_dir, "theauditor.sarif")

    # Staleness check (skip if --force or --stdout)
    if not force and not use_stdout and os.path.isfile(sarif_path) and csv_path:
        is_current, reason = check_staleness(sarif_path, db_path, csv_path)
        if is_current:
            print("SARIF is current (%s), skipping. Use --force to regenerate." % reason,
                  file=sys.stderr)
            sys.exit(0)
        else:
            print("Stale SARIF detected: %s. Regenerating..." % reason, file=sys.stderr)

    # Convert
    sarif = convert_db_to_sarif(db_path, language=language, benchmark_dir=benchmark_dir,
                                csv_path=csv_path)

    if use_stdout:
        json.dump(sarif, sys.stdout, indent=2)
        print()
    else:
        with open(sarif_path, "w", encoding="utf-8") as f:
            json.dump(sarif, f, indent=2)
            f.write("\n")
        result_count = len(sarif["runs"][0]["results"])
        print("Wrote %s (%d findings)" % (sarif_path, result_count), file=sys.stderr)


if __name__ == "__main__":
    main()
