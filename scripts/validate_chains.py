#!/usr/bin/env python3
"""Chain Detection Benchmark Fidelity Validator v3.0

Validates the chain benchmark's structural integrity using the same
L1-L5 fidelity system as the other benchmark validators.

  L1 -- Structural integrity (CSV <-> scenario directory cross-reference)
  L2 -- Roundtrip fidelity (files exist on disk, variants are paired)
  L3 -- Schema validation (required fields, valid CWEs, valid categories)
  L4 -- Anti-target-leakage (no comments, no docstrings, no CWE refs)
  L5 -- Scoring pipeline readiness (CHAIN_RULE_MAP coverage)

Exit 0 if all checks pass, 1 if any ERRORS, 2 if only WARNINGS.
No dependencies -- stdlib only.
"""

import ast
import os
import re
import sys
from collections import defaultdict
from pathlib import Path

# ============================================================================
# Configuration
# ============================================================================
SCRIPT_DIR = Path(__file__).resolve().parent
BENCH_ROOT = SCRIPT_DIR.parent
CHAINS_DIR = BENCH_ROOT / "chains"
CSV_FILE = CHAINS_DIR / "expectedresults-0.2.1.csv"
BENCHMARK_PY = SCRIPT_DIR / "convert_theauditor.py"
SCENARIOS_DIR = CHAINS_DIR / "scenarios"

VALID_CWES = {
    22, 78, 79, 89, 94, 113, 200, 269, 276, 287, 306, 327, 352, 362,
    384, 434, 502, 601, 611, 732, 798, 862, 863, 915, 918, 942,
}

VALID_CATEGORIES = {
    "unauth_injection",
    "ssrf_pivot",
    "compound_injection",
    "multi_stage",
    "privesc_chain",
    "idor_data_leak",
    "race_condition_bypass",
    "path_traversal_to_read",
    "open_redirect_to_phish",
    "xxe_to_file_read",
    "csrf_to_state_change",
    "header_injection_to_cache_poison",
    "weak_crypto_to_forge",
    "mass_assign_to_privesc",
    "info_leak_to_account_takeover",
    "template_injection_to_rce",
    "hardcoded_creds_to_access",
    "insecure_file_perms_to_tamper",
    "cors_miscfg_to_data_theft",
    "session_fixation_to_hijack",
}

LEAKY_PATTERNS = [
    re.compile(r"CWE-\d+"),
    re.compile(r"VULNERABLE|SAFE variant|IDENTICAL between"),
    re.compile(r"#\s*(BUG|FIXED|VULN|TOCTOU|VULNERABLE|SAFE)\s*:"),
    re.compile(r"--\s*injectable"),
    re.compile(r"vuln-code-snippet"),
]

errors = []
warnings = []


# ============================================================================
# L1: Structural Integrity
# ============================================================================
def parse_csv():
    """Parse expectedresults CSV."""
    entries = {}
    seen_keys = {}

    with open(CSV_FILE, "r", encoding="utf-8") as f:
        for line_num, line in enumerate(f, 1):
            stripped = line.strip()
            if not stripped or stripped.startswith("#"):
                continue

            parts = stripped.split(",")
            if len(parts) < 4:
                errors.append(f"L1 CSV line {line_num}: expected 4 columns, got {len(parts)}")
                continue

            key = parts[0].strip()
            category = parts[1].strip()
            vulnerable_str = parts[2].strip().lower()
            cwe_str = parts[3].strip()

            if key in seen_keys:
                errors.append(
                    f"L1 CSV duplicate key '{key}' at line {line_num} "
                    f"(first at line {seen_keys[key]})"
                )
            seen_keys[key] = line_num

            try:
                cwe = int(cwe_str)
            except ValueError:
                errors.append(f"L3 Invalid CWE value '{cwe_str}' for key '{key}' at CSV line {line_num}")
                cwe = -1

            entries[key] = {
                "key": key,
                "category": category,
                "vulnerable": vulnerable_str == "true",
                "cwe": cwe,
                "_csv_line": line_num,
            }

    return entries


def scan_scenario_keys():
    """Derive test case keys from scenario directory structure.
    scenarios/scenario_NNNN/variant_a/ -> ChainScenarioNNNNA
    scenarios/scenario_NNNN/variant_b/ -> ChainScenarioNNNNB
    """
    keys = set()

    if not SCENARIOS_DIR.is_dir():
        errors.append(f"L1 Scenarios directory not found: {SCENARIOS_DIR}")
        return keys

    for scenario_dir in sorted(SCENARIOS_DIR.iterdir()):
        if not scenario_dir.is_dir():
            continue
        m = re.match(r"scenario_(\d{4})$", scenario_dir.name)
        if not m:
            warnings.append(f"L1 Non-standard scenario dir: {scenario_dir.name}")
            continue

        num = m.group(1)
        for variant in ("variant_a", "variant_b"):
            variant_dir = scenario_dir / variant
            if variant_dir.is_dir():
                letter = variant[-1].upper()
                keys.add("ChainScenario%s%s" % (num, letter))

    return keys


# ============================================================================
# L2: Roundtrip Fidelity
# ============================================================================
def check_roundtrip(csv_entries, scenario_keys):
    """Verify scenario directories exist and have paired variants."""
    # Group keys by scenario number
    scenarios = defaultdict(set)
    for key in scenario_keys:
        m = re.match(r"ChainScenario(\d{4})([AB])$", key)
        if m:
            scenarios[m.group(1)].add(m.group(2))

    for num, variants in sorted(scenarios.items()):
        if variants != {"A", "B"}:
            errors.append(
                f"L2 Scenario {num} has incomplete variants: {variants} (need A and B)"
            )

        scenario_dir = SCENARIOS_DIR / ("scenario_%s" % num)
        for v in ("variant_a", "variant_b"):
            variant_dir = scenario_dir / v
            if variant_dir.is_dir():
                py_files = list(variant_dir.glob("*.py"))
                if not py_files:
                    errors.append(f"L2 Empty variant: {variant_dir}")
            else:
                errors.append(f"L2 Missing variant dir: {variant_dir}")


# ============================================================================
# L3: Schema Validation
# ============================================================================
def check_schema(csv_entries):
    """Validate required fields, CWE values, and category consistency."""
    all_categories = set()

    for key, info in csv_entries.items():
        cwe = info.get("cwe")
        if cwe is not None and cwe not in VALID_CWES:
            warnings.append(
                f"L3 CWE {cwe} for key '{key}' is not in the known chain CWE set"
            )

        cat = info.get("category")
        if cat:
            all_categories.add(cat)
            if cat not in VALID_CATEGORIES:
                errors.append(
                    f"L3 Unknown category '{cat}' for key '{key}' "
                    f"(valid: {', '.join(sorted(VALID_CATEGORIES))})"
                )

    return all_categories


# ============================================================================
# L4: Anti-Target-Leakage
# ============================================================================
def check_leakage():
    """Verify no comments, docstrings, CWE references, or annotations in test files."""
    leakage_count = 0

    for py_file in sorted(SCENARIOS_DIR.rglob("*.py")):
        rel = str(py_file.relative_to(CHAINS_DIR))
        try:
            content = py_file.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue

        for pat in LEAKY_PATTERNS:
            if pat.search(content):
                errors.append(f"L4 Target leakage in {rel}: matches /{pat.pattern}/")
                leakage_count += 1
                break

        try:
            tree = ast.parse(content)
            for node in ast.walk(tree):
                if isinstance(node, (ast.Module, ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
                    if (node.body and isinstance(node.body[0], ast.Expr)
                            and isinstance(node.body[0].value, ast.Constant)
                            and isinstance(node.body[0].value.value, str)):
                        errors.append(f"L4 Docstring in {rel}:{node.body[0].lineno}")
                        leakage_count += 1
                        break
        except SyntaxError:
            errors.append(f"L4 Syntax error in {rel}")
            leakage_count += 1

        for i, line in enumerate(content.splitlines(), 1):
            stripped = line.lstrip()
            if stripped.startswith("#"):
                errors.append(f"L4 Comment in {rel}:{i}: {stripped[:60]}")
                leakage_count += 1
                break

    return leakage_count


# ============================================================================
# L5: Scoring Pipeline Readiness
# ============================================================================
def check_scoring_pipeline(all_categories):
    """Verify all chain categories are mapped in convert_theauditor.py."""
    if not BENCHMARK_PY.exists():
        warnings.append("L5 convert_theauditor.py not found - cannot verify scoring pipeline")
        return

    with open(BENCHMARK_PY, "r", encoding="utf-8") as f:
        scoring_content = f.read()

    if "resolve_chain_key_from_path" not in scoring_content:
        errors.append("L5 convert_theauditor.py missing resolve_chain_key_from_path")

    mapped_categories = set()
    for m in re.finditer(r'"([a-z][a-z0-9_]+)"', scoring_content):
        mapped_categories.add(m.group(1))

    for cat in sorted(all_categories):
        if cat not in mapped_categories:
            warnings.append(
                f"L5 Category '{cat}' exists in ground truth but is not mapped "
                f"in convert_theauditor.py CHAIN_RULE_MAP or CHAIN_SINK_MAP"
            )


# ============================================================================
# Report
# ============================================================================
def print_report(csv_entries):
    """Print the fidelity report."""
    categories = defaultdict(lambda: {"tp": 0, "tn": 0})
    for key, info in csv_entries.items():
        cat = info.get("category", "unknown")
        if info.get("vulnerable", False):
            categories[cat]["tp"] += 1
        else:
            categories[cat]["tn"] += 1

    total_tp = sum(c["tp"] for c in categories.values())
    total_tn = sum(c["tn"] for c in categories.values())
    total = total_tp + total_tn

    print(f"CSV entries:       {len(csv_entries)}")
    print(f"Total chains:      {total_tp} exploitable / {total_tn} mitigated")
    if total > 0:
        print(f"Balance:           {total_tp * 100.0 / total:.1f}% / {total_tn * 100.0 / total:.1f}%")
    print()

    print(f"{'Category':<24} {'CWE':>5} {'Vuln':>5} {'Safe':>5} {'Total':>6}")
    print("-" * 50)

    cat_cwes = {}
    for key, info in csv_entries.items():
        cat = info.get("category")
        cwe = info.get("cwe")
        if cat and cwe:
            cat_cwes.setdefault(cat, set()).add(cwe)

    for cat in sorted(categories.keys()):
        tp = categories[cat]["tp"]
        tn = categories[cat]["tn"]
        cwes = cat_cwes.get(cat, set())
        cwe_str = ",".join(str(c) for c in sorted(cwes))
        print(f"{cat:<24} {cwe_str:>5} {tp:>5} {tn:>5} {tp + tn:>6}")

    print("-" * 50)
    print(f"{'TOTAL':<24} {'':>5} {total_tp:>5} {total_tn:>5} {total:>6}")
    print()


def main():
    print("=" * 64)
    print("  Chain Detection Benchmark Fidelity Validator v3.0")
    print("  L1-L5 fidelity system (annotation-free)")
    print("=" * 64)
    print()

    # --- L1 ---
    print("[L1] Structural Integrity (CSV <-> scenario directory cross-reference)")
    csv_entries = parse_csv()
    scenario_keys = scan_scenario_keys()

    csv_keys = set(csv_entries.keys())

    for key in sorted(csv_keys - scenario_keys):
        errors.append(f"L1 Orphan CSV: '{key}' in ground truth but no scenario directory")
    for key in sorted(scenario_keys - csv_keys):
        errors.append(f"L1 Orphan directory: '{key}' on disk but no CSV entry")

    l1_errors = len(errors)
    print(f"  CSV keys: {len(csv_keys)}, Directory keys: {len(scenario_keys)}")
    print(f"  Result: {l1_errors} errors found")
    print()

    # --- L2 ---
    print("[L2] Roundtrip Fidelity (paired variants, files on disk)")
    check_roundtrip(csv_entries, scenario_keys)
    l2_errors = len(errors) - l1_errors
    print(f"  Result: {l2_errors} errors found")
    print()

    # --- L3 ---
    print("[L3] Schema Validation (CWEs, categories)")
    all_categories = check_schema(csv_entries)
    l3_errors = len(errors) - l1_errors - l2_errors
    print(f"  Categories: {len(all_categories)} ({', '.join(sorted(all_categories))})")
    print(f"  Result: {l3_errors} errors found")
    print()

    # --- L4 ---
    print("[L4] Anti-Target-Leakage (no comments, no docstrings, no annotations)")
    leakage = check_leakage()
    l4_errors = len(errors) - l1_errors - l2_errors - l3_errors
    print(f"  Result: {l4_errors} errors found")
    print()

    # --- L5 ---
    print("[L5] Scoring Pipeline Readiness")
    check_scoring_pipeline(all_categories)
    l5_warnings = len(warnings)
    print(f"  Result: {l5_warnings} warnings")
    print()

    # --- Summary ---
    print("=" * 64)
    print("  SUMMARY")
    print("=" * 64)
    print()

    print_report(csv_entries)

    if errors:
        print(f"ERRORS: {len(errors)}")
        for err in errors:
            print(f"  [ERROR] {err}")
        print()

    if warnings:
        print(f"WARNINGS: {len(warnings)}")
        for warn in warnings:
            print(f"  [WARN]  {warn}")
        print()

    if errors:
        print("RESULT: FAIL")
        return 1
    elif warnings:
        print("RESULT: PASS WITH WARNINGS")
        return 2
    else:
        print("RESULT: PASS")
        return 0


if __name__ == "__main__":
    sys.exit(main())
