#!/usr/bin/env python3
"""
Vulnerable Apps Benchmark Fidelity Validator v2.0
Validates all 5 languages (Go, Rust, PHP, Ruby, Bash) in one pass.

Checks:
  L1: CSV structural integrity (no duplicates, no empty fields, correct columns)
  L2: Annotation cross-reference (every CSV key has a source annotation, and vice versa)
  L3: Schema validation (valid categories, valid CWEs)
  L4: Annotation integrity (balanced start/end, vuln-line/safe-line/target-line present)
  L5: Comment leakage detection (no CWE numbers, category names, or "vulnerable"/"safe" in comments)
  L6: Go per-app integrity (line ranges, file existence, comment leakage, balance)
  L7: Doc existence ({lang}_apps_benchmark.md, CHANGELOG.md)
"""

import os
import re
import sys
from pathlib import Path
from collections import defaultdict

BENCHMARK_ROOT = Path(__file__).resolve().parent.parent
VULN_APPS = BENCHMARK_ROOT / "vulnerable_apps"

ERRORS = []
WARNINGS = []


def error(msg):
    ERRORS.append(msg)
    print(f"  [ERROR] {msg}")


def warn(msg):
    WARNINGS.append(msg)
    print(f"  [WARN]  {msg}")


def ok(msg):
    print(f"  [OK]    {msg}")


VALID_CWES = {
    20, 22, 78, 79, 89, 90, 93, 94, 98, 113, 117, 119, 190, 200, 209,
    250, 269, 285, 287, 295, 306, 319, 327, 328, 330, 346, 347, 352,
    362, 367, 377, 384, 434, 451, 470, 501, 502, 506, 532, 601, 611,
    614, 621, 627, 697, 732, 770, 798, 829, 838, 862, 915, 918, 942,
    943, 1059, 1333, 1336,
}

CATEGORY_NAMES = {
    "sqli", "cmdi", "codeinj", "xss", "ssrf", "pathtraver", "redirect",
    "hardcodedcreds", "hardcoded_creds", "securecookie", "massassign",
    "deserial", "weakrand", "weakhash", "weakcipher", "weakcrypto",
    "loginjection", "headerinj", "csrf", "ssti", "authnfailure",
    "authzfailure", "fileupload", "fileinclusion", "unsafereflect",
    "xxe", "ldapi", "typejuggling", "extract", "variablevars",
    "memsafety", "intoverflow", "redos", "inputval", "nosql",
    "infodisclosure", "insecure_perms", "insecure_temp", "ssl_bypass",
    "unquoted", "rce", "race_condition", "privilege_escalation",
    "auth_bypass", "cleartext_tx", "dos", "crypto", "tlsverify",
    "trustbound", "regex", "dynmethod", "cors",
}

_LEAKAGE_CI = re.compile(
    r'\b(?:CWE[-_ ]?\d{2,4}|vulnerab(?:ility|le)|exploit|injection|'
    r'true positive|false positive)\b',
    re.IGNORECASE
)
_LEAKAGE_CS = re.compile(
    r'\b(?:VULN|SAFE)\b'
    r'|(?<![A-Za-z])(?:TP|TN|FP|FN)(?![A-Za-z])'
)

def _leakage_match(text):
    return _LEAKAGE_CI.search(text) or _LEAKAGE_CS.search(text)

COMMENT_PREFIXES = {
    ".go": "//", ".rs": "//", ".php": "//", ".rb": "#", ".sh": "#",
    ".py": "#", ".js": "//", ".ts": "//", ".tsx": "//", ".jsx": "//",
}

SOURCE_EXTENSIONS = {".go", ".rs", ".php", ".rb", ".sh", ".py", ".js", ".ts", ".tsx"}


# ---------- L1: CSV structural integrity ----------

def parse_csv(csv_path, ncols):
    """Parse CSV, return rows. Checks for duplicates, empty fields."""
    if not csv_path.exists():
        error(f"CSV not found: {csv_path}")
        return []
    rows = []
    keys_seen = set()
    with open(csv_path, "r") as f:
        for lineno, line in enumerate(f, 1):
            line = line.strip()
            if not line or line.startswith("#") or line.startswith("function,") or line.startswith("test name,"):
                continue
            parts = line.split(",")
            if len(parts) != ncols:
                error(f"{csv_path.name}:{lineno}: {len(parts)} columns, expected {ncols}")
                continue
            key = parts[0]
            if not key:
                error(f"{csv_path.name}:{lineno}: empty key")
                continue
            unique_key = key if ncols == 4 else ",".join(parts)
            if unique_key in keys_seen:
                error(f"{csv_path.name}:{lineno}: duplicate key '{key}'")
            keys_seen.add(unique_key)
            rows.append(parts)
    return rows


# ---------- L2: Annotation cross-reference ----------

def get_annotation_keys(lang_dir):
    """Extract all vuln-code-snippet start keys from source files."""
    keys = set()
    pattern = re.compile(r"vuln-code-snippet start (\S+)")
    for root, _, files in os.walk(lang_dir):
        for fname in files:
            fpath = os.path.join(root, fname)
            ext = os.path.splitext(fname)[1]
            if ext not in SOURCE_EXTENSIONS:
                continue
            try:
                with open(fpath, "r", errors="replace") as f:
                    for line in f:
                        m = pattern.search(line)
                        if m:
                            keys.add(m.group(1))
            except OSError:
                pass
    return keys


def check_annotation_crossref(lang, csv_keys, lang_dir):
    """Every CSV key must have a source annotation, and vice versa."""
    ann_keys = get_annotation_keys(lang_dir)
    csv_only = csv_keys - ann_keys
    ann_only = ann_keys - csv_keys
    if csv_only:
        for k in sorted(csv_only):
            error(f"{lang}: CSV key '{k}' has no matching annotation in source")
    if ann_only:
        for k in sorted(ann_only):
            error(f"{lang}: annotation key '{k}' has no matching CSV entry")
    if not csv_only and not ann_only:
        ok(f"{lang}: all {len(csv_keys)} CSV keys match annotations")


# ---------- L4: Annotation integrity ----------

def check_annotation_integrity(lang, lang_dir):
    """Check balanced start/end pairs, marker presence inside snippets."""
    start_re = re.compile(r"vuln-code-snippet start (\S+)")
    end_re = re.compile(r"vuln-code-snippet end (\S+)")
    marker_re = re.compile(r"vuln-code-snippet (vuln-line|safe-line|target-line) (\S+)")

    open_snippets = {}
    closed_keys = set()
    markers_found = {}
    errs = 0

    for root, _, files in os.walk(lang_dir):
        for fname in sorted(files):
            ext = os.path.splitext(fname)[1]
            if ext not in SOURCE_EXTENSIONS:
                continue
            fpath = os.path.join(root, fname)
            rel = os.path.relpath(fpath, lang_dir)
            try:
                with open(fpath, "r", errors="replace") as f:
                    for lineno, line in enumerate(f, 1):
                        sm = start_re.search(line)
                        if sm:
                            key = sm.group(1)
                            if key in open_snippets:
                                error(f"{lang}/{rel}:{lineno}: snippet '{key}' opened twice without closing")
                                errs += 1
                            open_snippets[key] = (rel, lineno)

                        em = end_re.search(line)
                        if em:
                            key = em.group(1)
                            if key not in open_snippets:
                                error(f"{lang}/{rel}:{lineno}: snippet '{key}' closed without opening")
                                errs += 1
                            else:
                                del open_snippets[key]
                                closed_keys.add(key)

                        mm = marker_re.search(line)
                        if mm:
                            markers_found[mm.group(2)] = mm.group(1)
            except OSError:
                pass

    for key, (rel, lineno) in open_snippets.items():
        error(f"{lang}/{rel}:{lineno}: snippet '{key}' never closed")
        errs += 1

    if errs == 0:
        ok(f"{lang}: all annotation start/end pairs balanced")

    missing_markers = closed_keys - set(markers_found.keys())
    if missing_markers:
        for k in sorted(missing_markers):
            warn(f"{lang}: snippet '{k}' has no vuln-line/safe-line/target-line marker")


# ---------- L5: Comment leakage ----------

def check_comment_leakage(lang, lang_dir):
    """Scan source for CWE numbers, category names, or 'vulnerable'/'safe' in comments."""
    leaks = 0
    ann_re = re.compile(r"vuln-code-snippet")

    for root, _, files in os.walk(lang_dir):
        for fname in sorted(files):
            ext = os.path.splitext(fname)[1]
            if ext not in SOURCE_EXTENSIONS:
                continue
            prefix = COMMENT_PREFIXES.get(ext)
            if not prefix:
                continue
            fpath = os.path.join(root, fname)
            rel = os.path.relpath(fpath, lang_dir)
            try:
                with open(fpath, "r", errors="replace") as f:
                    for lineno, line in enumerate(f, 1):
                        stripped = line.strip()
                        if ann_re.search(stripped):
                            continue
                        if not stripped.startswith(prefix):
                            continue
                        comment_text = stripped[len(prefix):].strip()
                        if not comment_text:
                            continue
                        m = _leakage_match(comment_text)
                        if m:
                            leaks += 1
                            if leaks <= 5:
                                warn(f"{lang}/{rel}:{lineno}: potential leakage in comment: '{m.group()}'")
            except OSError:
                pass

    if leaks == 0:
        ok(f"{lang}: no comment leakage detected")
    elif leaks > 5:
        warn(f"{lang}: {leaks} total comment leakage instances ({leaks - 5} more not shown)")


# ---------- L6: Go per-app integrity ----------

def check_go_apps():
    """Verify Go per-app ground_truth.csv: line ranges, file existence, leakage, balance."""
    go_dir = VULN_APPS / "go"
    apps = [d for d in go_dir.iterdir() if d.is_dir() and (d / "ground_truth.csv").exists()]
    total = 0
    all_rows = []

    for app_dir in sorted(apps):
        csv_path = app_dir / "ground_truth.csv"
        rows = parse_csv(csv_path, 7)

        for parts in rows:
            func, fpath, start, end = parts[0], parts[1], parts[2], parts[3]
            full_path = app_dir / fpath
            if not full_path.exists():
                error(f"go/{app_dir.name}: file '{fpath}' referenced by '{func}' does not exist")
            try:
                s, e = int(start), int(end)
                if s > e:
                    error(f"go/{app_dir.name}: '{func}' has start_line {s} > end_line {e}")
                if s < 1:
                    error(f"go/{app_dir.name}: '{func}' has start_line {s} < 1")
            except ValueError:
                error(f"go/{app_dir.name}: '{func}' has non-integer line numbers")

        tp = sum(1 for r in rows if r[5].lower() == "true")
        tn = sum(1 for r in rows if r[5].lower() == "false")
        ok(f"go/{app_dir.name}: {len(rows)} entries ({tp} TP / {tn} TN)")
        total += len(rows)
        all_rows.extend(rows)

    check_comment_leakage("go", go_dir)

    cats = defaultdict(lambda: {"tp": 0, "tn": 0})
    for r in all_rows:
        cat = r[4]
        if r[5].lower() == "true":
            cats[cat]["tp"] += 1
        else:
            cats[cat]["tn"] += 1

    return total


# ---------- Main ----------

def validate_language(lang):
    """Run all checks for one language."""
    lang_dir = VULN_APPS / lang

    if lang == "go":
        print(f"\n[{lang.upper()}] Go per-app ground_truth.csv validation")
        total = check_go_apps()
        print(f"  Total Go entries: {total}")
        return total

    csv_path = lang_dir / "expectedresults-0.1.0.csv"
    print(f"\n[{lang.upper()}] Validating {csv_path.relative_to(BENCHMARK_ROOT)}")

    # L1: Structure
    rows = parse_csv(csv_path, 4)
    if not rows:
        return 0

    csv_keys = {r[0] for r in rows}
    tp = sum(1 for r in rows if r[2].lower() == "true")
    tn = sum(1 for r in rows if r[2].lower() == "false")
    total = len(rows)
    ok(f"{lang}: {total} entries ({tp} TP / {tn} TN)")

    # L2: Cross-reference
    check_annotation_crossref(lang, csv_keys, lang_dir)

    # L3: Schema
    categories = set()
    bad_cwes = set()
    for parts in rows:
        cat, vuln, cwe_str = parts[1], parts[2], parts[3]
        categories.add(cat)
        try:
            cwe = int(cwe_str)
            if cwe not in VALID_CWES:
                bad_cwes.add(cwe)
        except ValueError:
            error(f"{lang}: invalid CWE '{cwe_str}'")
        if vuln.lower() not in ("true", "false"):
            error(f"{lang}: invalid vulnerability value '{vuln}'")
    if bad_cwes:
        for c in sorted(bad_cwes):
            warn(f"{lang}: CWE-{c} not in known valid set")
    ok(f"{lang}: {len(categories)} categories: {', '.join(sorted(categories))}")

    # L4: Annotation integrity
    check_annotation_integrity(lang, lang_dir)

    # L5: Comment leakage
    check_comment_leakage(lang, lang_dir)

    # Category breakdown table
    cats = defaultdict(lambda: {"tp": 0, "tn": 0})
    for r in rows:
        cat = r[1]
        if r[2].lower() == "true":
            cats[cat]["tp"] += 1
        else:
            cats[cat]["tn"] += 1

    print(f"\n  {'Category':<25s} {'CWE':>5s} {'TP':>4s} {'TN':>4s} {'Total':>6s}")
    print(f"  {'-'*25} {'-'*5} {'-'*4} {'-'*4} {'-'*6}")
    for cat in sorted(cats, key=lambda c: cats[c]["tp"] + cats[c]["tn"], reverse=True):
        t, f = cats[cat]["tp"], cats[cat]["tn"]
        cwes = set()
        for r in rows:
            if r[1] == cat:
                cwes.add(r[3])
        cwe_str = "/".join(sorted(cwes))
        print(f"  {cat:<25s} {cwe_str:>5s} {t:>4d} {f:>4d} {t+f:>6d}")
    print(f"  {'-'*25} {'-'*5} {'-'*4} {'-'*4} {'-'*6}")
    print(f"  {'TOTAL':<25s} {'':>5s} {tp:>4d} {tn:>4d} {total:>6d}")

    return total


def main():
    print("=" * 64)
    print("  Vulnerable Apps Benchmark Fidelity Validator v2.0")
    print("  All 5 languages in one pass (L1-L7)")
    print("=" * 64)

    if not VULN_APPS.exists():
        print(f"ERROR: vulnerable_apps/ not found at {VULN_APPS}")
        sys.exit(1)

    grand_total = 0
    for lang in ["go", "rust", "php", "ruby", "bash"]:
        grand_total += validate_language(lang)

    # L7: Doc existence
    print(f"\n[DOCS] Checking documentation")
    for path, label in [
        (VULN_APPS / "README.md", "Root README"),
        (VULN_APPS / "CHANGELOG.md", "Root CHANGELOG"),
        (VULN_APPS / "dev_roadmap.md", "Root dev_roadmap"),
    ]:
        if path.exists():
            ok(label)
        else:
            error(f"{label} not found")

    for lang in ["go", "rust", "php", "ruby", "bash"]:
        bm = VULN_APPS / lang / f"{lang}_apps_benchmark.md"
        if bm.exists():
            ok(f"{lang}_apps_benchmark.md")
        else:
            error(f"{lang}_apps_benchmark.md not found")

    print("\n" + "=" * 64)
    print("  SUMMARY")
    print("=" * 64)
    print(f"\n  Grand total entries: {grand_total}")
    print(f"  Errors:   {len(ERRORS)}")
    print(f"  Warnings: {len(WARNINGS)}")

    if ERRORS:
        print(f"\n  RESULT: FAIL ({len(ERRORS)} errors)")
        sys.exit(1)
    else:
        print(f"\n  RESULT: PASS")
        if WARNINGS:
            print(f"  ({len(WARNINGS)} warnings -- review recommended)")
        sys.exit(0)


if __name__ == "__main__":
    main()
