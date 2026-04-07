#!/usr/bin/env python3
"""Chain benchmark anti-target-leakage migration: v0.2.0 -> v0.2.1

Strips all docstrings and comments from chain scenario files, renames
directories to opaque names, updates vuln-code-snippet annotations to
use opaque keys, and generates updated CSV.

Run from gorustbash_benchmark root:
    python3 scripts/migrate_chains_leakage.py
"""

import ast
import json
import os
import random
import re
import shutil
import sys
from pathlib import Path

try:
    import libcst as cst
except ImportError:
    print("ERROR: LibCST required. Install: pip install libcst")
    sys.exit(1)

SCRIPT_DIR = Path(__file__).resolve().parent
BENCH_ROOT = SCRIPT_DIR.parent
CHAINS_DIR = BENCH_ROOT / "chains"
SCENARIOS_DIR = CHAINS_DIR / "scenarios"
MIGRATION_DIR = SCENARIOS_DIR / "migration"
OLD_CSV = CHAINS_DIR / "expectedresults-0.2.0.csv"
NEW_CSV = CHAINS_DIR / "expectedresults-0.2.1.csv"
RENAME_MAP_FILE = CHAINS_DIR / "rename_map.json"

SEED = 20260408

PAT_SNIPPET_START = re.compile(r"vuln-code-snippet\s+start\s+(\S+)")
PAT_SNIPPET_END = re.compile(r"vuln-code-snippet\s+end\s+(\S+)")


# ============================================================================
# LibCST Transformers (adapted from TheAuditorV2/scripts/purge_comments_v3.py)
# ============================================================================


class DocstringRemover(cst.CSTTransformer):
    """Remove ALL docstrings (module, function, class)."""

    def _is_docstring(self, node):
        if not isinstance(node, cst.SimpleStatementLine):
            return False
        if len(node.body) != 1:
            return False
        stmt = node.body[0]
        if not isinstance(stmt, cst.Expr):
            return False
        return isinstance(
            stmt.value,
            (cst.SimpleString, cst.ConcatenatedString, cst.FormattedString),
        )

    def _remove_docstring_from_body(self, body):
        if not isinstance(body, cst.IndentedBlock) or not body.body:
            return body
        if not self._is_docstring(body.body[0]):
            return body
        remaining = list(body.body[1:])
        if not remaining:
            remaining = [cst.SimpleStatementLine(body=[cst.Pass()])]
        return body.with_changes(body=remaining)

    def leave_Module(self, original_node, updated_node):
        if updated_node.body and self._is_docstring(updated_node.body[0]):
            new_body = list(updated_node.body[1:])
            return updated_node.with_changes(body=new_body)
        return updated_node

    def leave_FunctionDef(self, original_node, updated_node):
        new_body = self._remove_docstring_from_body(updated_node.body)
        if new_body is not updated_node.body:
            return updated_node.with_changes(body=new_body)
        return updated_node

    def leave_ClassDef(self, original_node, updated_node):
        new_body = self._remove_docstring_from_body(updated_node.body)
        if new_body is not updated_node.body:
            return updated_node.with_changes(body=new_body)
        return updated_node


class CommentStripper(cst.CSTTransformer):
    """Strip all # comments except vuln-code-snippet annotations (updated)."""

    def __init__(self, key_map):
        super().__init__()
        self.key_map = key_map

    def _update_annotation(self, comment_value):
        updated = comment_value
        for old_key, new_key in self.key_map.items():
            if old_key in updated:
                updated = updated.replace(old_key, new_key)
                break
        updated = updated.replace("vuln-line", "target-line")
        updated = updated.replace("safe-line", "target-line")
        return updated

    def leave_EmptyLine(self, original_node, updated_node):
        if updated_node.comment is None:
            return updated_node
        text = updated_node.comment.value
        if "vuln-code-snippet" in text:
            new_text = self._update_annotation(text)
            return updated_node.with_changes(
                comment=updated_node.comment.with_changes(value=new_text)
            )
        return updated_node.with_changes(comment=None)

    def leave_TrailingWhitespace(self, original_node, updated_node):
        if updated_node.comment is None:
            return updated_node
        text = updated_node.comment.value
        if "vuln-code-snippet" in text:
            new_text = self._update_annotation(text)
            return updated_node.with_changes(
                comment=updated_node.comment.with_changes(value=new_text)
            )
        return updated_node.with_changes(comment=None)


# ============================================================================
# Post-processing
# ============================================================================


def collapse_blank_lines(code):
    """Collapse consecutive blank lines to at most one. Strip leading blanks."""
    lines = code.split("\n")
    result = []
    prev_blank = False
    for line in lines:
        is_blank = line.strip() == ""
        if is_blank and prev_blank:
            continue
        result.append(line)
        prev_blank = is_blank
    while result and result[0].strip() == "":
        result.pop(0)
    while result and result[-1].strip() == "":
        result.pop()
    out = "\n".join(result)
    if out and not out.endswith("\n"):
        out += "\n"
    return out


# ============================================================================
# Scanning & mapping
# ============================================================================


def scan_scenario_annotations():
    """Scan scenario files for vuln-code-snippet keys.
    Returns: {scenario_dir_name: {"vuln": csv_key, "safe": csv_key}}
    """
    scenario_keys = {}

    for scenario_dir in sorted(SCENARIOS_DIR.iterdir()):
        if not scenario_dir.is_dir() or scenario_dir.name == "migration":
            continue

        keys = {}
        for variant in ("vuln", "safe"):
            variant_dir = scenario_dir / variant
            if not variant_dir.is_dir():
                continue
            for py_file in sorted(variant_dir.glob("*.py")):
                content = py_file.read_text(encoding="utf-8", errors="replace")
                m = PAT_SNIPPET_START.search(content)
                if m:
                    keys[variant] = m.group(1)
                    break

        if "vuln" in keys and "safe" in keys:
            scenario_keys[scenario_dir.name] = keys
        else:
            print("  WARNING: %s missing keys: %s" % (scenario_dir.name, keys))

    return scenario_keys


def build_rename_map(scenario_keys):
    """Build the full rename map with seeded shuffle.
    Returns: (rename_map_dict, flat_key_map)
    """
    scenario_names = sorted(scenario_keys.keys())

    shuffled = scenario_names[:]
    random.seed(SEED)
    random.shuffle(shuffled)

    rename_map = {"seed": SEED, "scenario_count": len(shuffled), "scenarios": {}}
    key_map = {}

    for new_idx, old_name in enumerate(shuffled, 1):
        new_dir = "scenario_%04d" % new_idx

        vuln_is_a = random.choice([True, False])

        old_vuln_key = scenario_keys[old_name]["vuln"]
        old_safe_key = scenario_keys[old_name]["safe"]

        if vuln_is_a:
            new_vuln_key = "ChainScenario%04dA" % new_idx
            new_safe_key = "ChainScenario%04dB" % new_idx
            vuln_variant = "variant_a"
            safe_variant = "variant_b"
        else:
            new_vuln_key = "ChainScenario%04dB" % new_idx
            new_safe_key = "ChainScenario%04dA" % new_idx
            vuln_variant = "variant_b"
            safe_variant = "variant_a"

        rename_map["scenarios"][old_name] = {
            "new_dir": new_dir,
            "vuln_variant": vuln_variant,
            "safe_variant": safe_variant,
            "keys": {
                old_vuln_key: new_vuln_key,
                old_safe_key: new_safe_key,
            },
        }

        key_map[old_vuln_key] = new_vuln_key
        key_map[old_safe_key] = new_safe_key

    return rename_map, key_map


# ============================================================================
# File processing
# ============================================================================


def process_file(source_path, dest_path, key_map):
    """Process one .py file: strip docstrings + comments, update annotations."""
    source = source_path.read_text(encoding="utf-8", errors="replace")

    try:
        module = cst.parse_module(source)
    except cst.ParserSyntaxError as e:
        print("  SYNTAX ERROR in %s: %s" % (source_path, e))
        dest_path.parent.mkdir(parents=True, exist_ok=True)
        dest_path.write_text(source, encoding="utf-8")
        return False

    module = module.visit(DocstringRemover())
    module = module.visit(CommentStripper(key_map))

    cleaned = collapse_blank_lines(module.code)

    dest_path.parent.mkdir(parents=True, exist_ok=True)
    with open(dest_path, "w", encoding="utf-8", newline="\n") as f:
        f.write(cleaned)
    return True


# ============================================================================
# Main
# ============================================================================


def main():
    print("=" * 64)
    print("  Chain Benchmark Anti-Target-Leakage Migration v0.2.0 -> v0.2.1")
    print("=" * 64)
    print()

    # ------------------------------------------------------------------
    # Phase 1: Build Rename Map
    # ------------------------------------------------------------------
    print("[Phase 1] Scanning annotations & building rename map (seed=%d)" % SEED)

    scenario_keys = scan_scenario_annotations()
    print("  Found %d scenarios with annotation pairs" % len(scenario_keys))

    if len(scenario_keys) != 250:
        print("  WARNING: Expected 250, got %d" % len(scenario_keys))

    rename_map, key_map = build_rename_map(scenario_keys)

    with open(RENAME_MAP_FILE, "w", encoding="utf-8", newline="\n") as f:
        json.dump(rename_map, f, indent=2, sort_keys=True)

    print("  Key mappings: %d" % len(key_map))
    print("  Rename map: %s" % RENAME_MAP_FILE)
    print()

    # ------------------------------------------------------------------
    # Phase 2: File Processing (LibCST)
    # ------------------------------------------------------------------
    print("[Phase 2] File Processing (docstrings + comments + annotation update)")

    if MIGRATION_DIR.exists():
        shutil.rmtree(MIGRATION_DIR)
    MIGRATION_DIR.mkdir()

    total_files = 0
    total_scenarios = 0

    for old_name, info in rename_map["scenarios"].items():
        new_dir = info["new_dir"]
        old_scenario = SCENARIOS_DIR / old_name

        for orig_variant, new_variant in [
            ("vuln", info["vuln_variant"]),
            ("safe", info["safe_variant"]),
        ]:
            src_dir = old_scenario / orig_variant
            dst_dir = MIGRATION_DIR / new_dir / new_variant

            if not src_dir.is_dir():
                print("  WARNING: Missing %s" % src_dir)
                continue

            for py_file in sorted(src_dir.glob("*.py")):
                dest = dst_dir / py_file.name
                process_file(py_file, dest, key_map)
                total_files += 1

        total_scenarios += 1

    print("  Processed %d scenarios, %d files" % (total_scenarios, total_files))
    print()

    # ------------------------------------------------------------------
    # Validation Gates
    # ------------------------------------------------------------------

    # Gate 1: Zero leakage
    print("  [Gate 1] Zero leakage check...")
    errors = []

    for py_file in sorted(MIGRATION_DIR.rglob("*.py")):
        content = py_file.read_text(encoding="utf-8", errors="replace")
        rel = str(py_file.relative_to(MIGRATION_DIR))

        try:
            tree = ast.parse(content)
            for node in ast.walk(tree):
                if isinstance(node, (ast.Module, ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
                    if (node.body and isinstance(node.body[0], ast.Expr)
                            and isinstance(node.body[0].value, ast.Constant)
                            and isinstance(node.body[0].value.value, str)):
                        errors.append("Docstring in %s:%d" % (rel, node.body[0].lineno))
        except SyntaxError:
            pass
        if "CWE-" in content:
            errors.append("CWE ref in %s" % rel)
        if re.search(r"VULNERABLE|SAFE variant|IDENTICAL", content):
            errors.append("Leaky label in %s" % rel)
        if re.search(r"# BUG:|# FIXED:|# VULN:|# TOCTOU:", content):
            errors.append("Leaky comment in %s" % rel)
        if re.search(r"chain_\w+_(vuln|safe)", content):
            errors.append("Old key in %s" % rel)

        for i, line in enumerate(content.splitlines(), 1):
            stripped = line.lstrip()
            if stripped.startswith("#") and "vuln-code-snippet" not in stripped:
                errors.append("Stray comment %s:%d: %s" % (rel, i, stripped[:60]))

    if errors:
        print("  FAILED (%d errors):" % len(errors))
        for e in errors[:30]:
            print("    %s" % e)
        if len(errors) > 30:
            print("    ... and %d more" % (len(errors) - 30))
        sys.exit(1)
    print("  Gate 1: PASS")

    # Gate 2: Structural integrity
    print("  [Gate 2] Structural integrity...")
    migration_scenarios = [d for d in MIGRATION_DIR.iterdir() if d.is_dir()]
    migration_files = list(MIGRATION_DIR.rglob("*.py"))

    if len(migration_scenarios) != len(scenario_keys):
        print(
            "  FAILED: expected %d scenarios, got %d"
            % (len(scenario_keys), len(migration_scenarios))
        )
        sys.exit(1)
    if len(migration_files) != total_files:
        print(
            "  FAILED: expected %d files, got %d" % (total_files, len(migration_files))
        )
        sys.exit(1)
    print(
        "  Gate 2: PASS (%d scenarios, %d files)"
        % (len(migration_scenarios), len(migration_files))
    )

    # Gate 3: Syntax validity
    print("  [Gate 3] Syntax validity...")
    syntax_errors = 0
    for py_file in migration_files:
        content = py_file.read_text(encoding="utf-8", errors="replace")
        try:
            ast.parse(content)
        except SyntaxError as e:
            syntax_errors += 1
            if syntax_errors <= 10:
                print(
                    "    %s: %s" % (py_file.relative_to(MIGRATION_DIR), e)
                )
    if syntax_errors:
        print("  FAILED: %d syntax errors" % syntax_errors)
        sys.exit(1)
    print("  Gate 3: PASS (%d files parsed)" % len(migration_files))

    # Gate 4: Annotation integrity
    print("  [Gate 4] Annotation integrity...")
    ann_starts = set()
    ann_ends = set()
    target_lines = set()

    for py_file in migration_files:
        content = py_file.read_text(encoding="utf-8", errors="replace")
        for line in content.splitlines():
            m = PAT_SNIPPET_START.search(line)
            if m:
                ann_starts.add(m.group(1))
            m = PAT_SNIPPET_END.search(line)
            if m:
                ann_ends.add(m.group(1))
            if "vuln-code-snippet" in line and "target-line" in line:
                m = re.search(r"target-line\s+(\S+)", line)
                if m:
                    target_lines.add(m.group(1))

    new_keys = set(key_map.values())
    gate4_ok = True

    if ann_starts != new_keys:
        print("  FAILED: start annotations mismatch (%d vs %d)" % (len(ann_starts), len(new_keys)))
        missing = new_keys - ann_starts
        if missing:
            print("    Missing: %s" % list(missing)[:5])
        gate4_ok = False
    if ann_ends != new_keys:
        print("  FAILED: end annotations mismatch (%d vs %d)" % (len(ann_ends), len(new_keys)))
        gate4_ok = False
    if target_lines != new_keys:
        print(
            "  FAILED: target-line mismatch (%d vs %d)"
            % (len(target_lines), len(new_keys))
        )
        gate4_ok = False

    if not gate4_ok:
        sys.exit(1)
    print(
        "  Gate 4: PASS (%d starts, %d ends, %d target-lines)"
        % (len(ann_starts), len(ann_ends), len(target_lines))
    )
    print()

    # ------------------------------------------------------------------
    # Phase 3: CSV Migration
    # ------------------------------------------------------------------
    print("[Phase 3] CSV Migration")

    with open(OLD_CSV, "r", encoding="utf-8") as f:
        old_lines = f.readlines()

    new_lines = []
    new_lines.append(
        "# test name,category,chain exploitable,CWE,Benchmark version: 0.2.1,%s\n"
        % "2026-04-08"
    )
    new_lines.append("# Chain Detection Benchmark v0.2.1 (anti-target-leakage migration)\n")
    new_lines.append(
        "# %d test cases, 20 categories, %d/%d exploitable/mitigated\n"
        % (len(key_map), len(key_map) // 2, len(key_map) // 2)
    )

    mapped = 0
    for line in old_lines:
        stripped = line.strip()
        if not stripped or stripped.startswith("#"):
            continue
        parts = stripped.split(",")
        old_key = parts[0].strip()

        if old_key in key_map:
            parts[0] = key_map[old_key]
            new_lines.append(",".join(parts) + "\n")
            mapped += 1
        else:
            print("  WARNING: unmapped key: %s" % old_key)

    with open(NEW_CSV, "w", encoding="utf-8", newline="\n") as f:
        f.writelines(new_lines)

    print("  Mapped %d entries" % mapped)
    print("  Written: %s" % NEW_CSV)

    # Gate 5: CSV integrity
    print("  [Gate 5] CSV integrity...")
    csv_keys = set()
    csv_true = 0
    csv_false = 0
    csv_cats = set()
    dupes = []

    with open(NEW_CSV, "r", encoding="utf-8") as f:
        for line in f:
            s = line.strip()
            if not s or s.startswith("#"):
                continue
            parts = s.split(",")
            key = parts[0].strip()
            if key in csv_keys:
                dupes.append(key)
            csv_keys.add(key)
            csv_cats.add(parts[1].strip())
            if parts[2].strip().lower() == "true":
                csv_true += 1
            else:
                csv_false += 1

    if dupes:
        print("  FAILED: duplicate keys: %s" % dupes[:5])
        sys.exit(1)
    if len(csv_keys) != len(key_map):
        print(
            "  FAILED: expected %d keys, got %d" % (len(key_map), len(csv_keys))
        )
        sys.exit(1)
    if len(csv_cats) != 20:
        print("  WARNING: expected 20 categories, got %d" % len(csv_cats))

    print(
        "  Gate 5: PASS (%d keys, %d categories, %d true / %d false, 0 duplicates)"
        % (len(csv_keys), len(csv_cats), csv_true, csv_false)
    )
    print()

    # ------------------------------------------------------------------
    # Summary
    # ------------------------------------------------------------------
    print("=" * 64)
    print("  MIGRATION READY FOR REVIEW")
    print("=" * 64)
    print()
    print("  Clean files:  %s/" % MIGRATION_DIR)
    print("  New CSV:      %s" % NEW_CSV)
    print("  Rename map:   %s" % RENAME_MAP_FILE)
    print()
    print("  Next steps (manual):")
    print("    1. Spot-check files in migration/")
    print("    2. Update validate_chains.py, chain_benchmark.md, CHANGELOG.md")
    print("    3. Run validate_chains.py against migration/ + new CSV")
    print("    4. Archive originals, swap migration/ into place")
    print()


if __name__ == "__main__":
    main()
