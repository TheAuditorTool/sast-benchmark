# Rust SAST Benchmark

## Methodology

Modeled after OWASP BenchmarkJava (the gold standard — 2,740 test cases, 100% achieved).

**Ground truth**: `expectedresults-0.1.csv` — CSV answer key (scoring authority). `rust_ground_truth.yml` provides descriptions.
**Scoring**: Youden's J (TPR - FPR) per CWE category. 0% = random guessing. +100% = perfect.

### Test Case Inventory

| Category | CWE | TP | TN | Total | Balance |
|----------|-----|----|----|-------|---------|
| sqli | 89 | 24 | 26 | 50 | 48/52 |
| cmdi | 78 | 14 | 16 | 30 | 47/53 |
| pathtraver | 22 | 14 | 14 | 28 | 50/50 |
| ssrf | 918 | 9 | 13 | 22 | 41/59 |
| memsafety | 119 | 8 | 12 | 20 | 40/60 |
| crypto | 327 | 9 | 11 | 20 | 45/55 |
| weakrand | 330 | 7 | 9 | 16 | 44/56 |
| xss | 79 | 5 | 11 | 16 | 31/69 |
| infodisclosure | 200+ | 8 | 8 | 16 | 50/50 |
| deser | 502 | 5 | 7 | 12 | 42/58 |
| intoverflow | 190 | 5 | 7 | 12 | 42/58 |
| redos | 1333 | 5 | 5 | 10 | 50/50 |
| inputval | 20 | 4 | 6 | 10 | 40/60 |
| **TOTAL** | | **117** | **145** | **262** | **45/55** |

**All 13 categories have TP AND TN.** Every category can measure both TPR and FPR. TP/TN ratio: 44/56 (Java gold standard: 52/48). FPR measurable for 100% of test cases.

### Frameworks Covered

| Framework | Apps | .rs Files |
|-----------|------|-----------|
| actix-web | rust_taint_app, deepflow-rust, rust_backend, rust_calorie_app, anarchy_commerce | ~60 |
| axum | rust_jobqueue | ~39 |
| rocket | rocket_test | 1 |
| warp | warp_test | 1 |

---

## App Catalog

### 1. rust_taint_app (actix-web + sqlx/rusqlite)
**Purpose:** Intentional taint flow test app. Every handler is a taint source, downstream modules are sinks.
**Architecture:** `handlers.rs` -> `database.rs` / `commands.rs` / `files.rs` / `network.rs` / `unsafe_ops.rs`
**Security posture:** ~50% vulnerable, ~50% safe by design. Both patterns exist for SQL, file ops.
**Test cases:** 23 (Phase 1)

### 2. deepflow-rust (actix-web)
**Purpose:** Deep taint flow complexity testing. Pipeline depth, async boundaries, closure captures, iterator chains, trait dispatch.
**Architecture:** `handlers.rs` -> `pipeline.rs` / `async_flow.rs` / `closures.rs` / `iterators.rs` / `traits.rs` / `advanced.rs` -> `sinks.rs`
**Security posture:** Nearly ALL vulnerable by design. Tests flow tracing, not pattern detection. Many sinks are simulated (println! not real SQL).
**Scoring:** EXCLUDED from ground truth — value is taint stress testing, not TP/TN benchmark scoring.

### 3. rust_backend (actix-web)
**Purpose:** Backend with intentional vulnerability categories (SQLi, ReDoS, weak crypto, race condition, integer overflow, deserialization, memory corruption).
**Architecture:** `handlers.rs` -> `vulnerable.rs` + `unsafe_ops.rs`
**Test cases:** 14 (Phase 1)

### 4. rocket_test (Rocket)
**Purpose:** Rocket framework extractors (Path, Form, Json, CookieJar, State). 7 vulnerable + 2 safe handlers.
**Test cases:** 9 (Phase 1)

### 5. warp_test (Warp)
**Purpose:** Warp filter combinators (path!, query, body::json, cookie, header). 8 vulnerable + 2 safe handlers.
**Test cases:** 10 (Phase 1)

### 6. rust_calorie_app (actix-web + sqlx) — REAL APP
**Purpose:** Production-style calorie tracker. NOT intentionally vulnerable — has NATURAL bugs alongside proper patterns.
**Architecture:** `handlers/` -> `validation/` -> `services/` -> `repositories/` -> SQLite
**Security posture:** Mostly safe. 3 SQLi in workout_repo + schedule_repo (format! with weak quote escaping). 15+ safe parameterized functions. Hardcoded JWT secret. Proper bcrypt + input validation.
**Key value:** Natural TN source. Safe repos (user_repo, meal_repo) are excellent false-positive tests. Workout/schedule vulns are REALISTIC — partial escaping that LOOKS safe but isn't.
**Test cases:** 12 (Phase 2)

### 7. rust_jobqueue (axum + rusqlite) — INTENTIONALLY VULNERABLE
**Purpose:** Job queue with every security antipattern. SQLi, path traversal, command injection, weak crypto, SSRF, info disclosure.
**Architecture:** API (axum) -> middleware -> handlers -> DB (rusqlite) -> workers -> backup
**Security posture:** 25+ intentional vulns across 6 CWE categories. Broken sanitization attempts (incomplete escaping, weak validation) make good edge cases.
**Test cases:** 18 (Phase 2)

### 8. anarchy_commerce (actix-web + reqwest)
**Purpose:** Microservices (payments + search). Hardcoded Stripe key, Elasticsearch injection, missing validation.
**Test cases:** 3 (Phase 2)

---

## Rule Coverage Matrix (verified against source code)

Maps each benchmark category to SAST rules that detect it.

| Category | CWE | Rule File | Track | Coverage |
|----------|-----|-----------|-------|----------|
| sqli | 89 | `rust/rust_injection_analyze.py` | taint+structural | FULL |
| cmdi | 78 | `rust/rust_injection_analyze.py` | taint+structural | FULL |
| pathtraver | 22 | `security/path_traversal_analyze.py` | taint | FULL |
| ssrf | 918 | `security/ssrf_analyze.py` | taint | FULL |
| memsafety | 119 | `rust/memory_safety.py` + `unsafe_analysis.py` + `ffi_boundary.py` | structural | FULL |
| xss | 79 | NONE | - | **GAP** (no .rs in xss rule target_extensions) |
| crypto | 327 | `rust/supply_chain.py` (dep-level only) | structural | **PARTIAL** (Cargo.toml only, not code-level) |
| weakrand | 330 | NONE | - | **GAP** |
| deser | 502 | NONE | - | **GAP** (patterns exist, no rule) |
| redos | 1333 | `security/redos_analyze.py` (no .rs) | - | **GAP** (LOW fix: add .rs) |
| intoverflow | 190 | `rust/integer_safety.py` | structural | **PARTIAL** (crypto/financial only) |
| infodisclosure | 200+ | NONE | - | **GAP** |
| inputval | 20 | NONE | - | **GAP** |

**22 of 262 test cases (8%) are in gap categories** -- expected to show as FN in baseline scoring.

---

## Current Scorecard

**Baseline: Pending. Human runs `aud full --offline` on this directory, then run scoring script below.**
**Scoring script: Fixed (v2) — taint_sinks removed, RULE_MAP expanded, path normalization added.**

```
Category         CWE    TP    FP    FN    TN      TPR     FPR   Score
----------------------------------------------------------------------
sqli             89     ?     ?     ?     ?     ?.?%    ?.?%  +?.?%
cmdi             78     ?     ?     ?     ?     ?.?%    ?.?%  +?.?%
pathtraver       22     ?     ?     ?     ?     ?.?%    ?.?%  +?.?%
ssrf             918    ?     ?     ?     ?     ?.?%    ?.?%  +?.?%
xss              79     ?     ?     ?     ?     ?.?%    ?.?%  +?.?%
crypto           327    ?     ?     ?     ?     ?.?%    ?.?%  +?.?%
weakrand         330    ?     ?     ?     ?     ?.?%    ?.?%  +?.?%
memsafety        119    ?     ?     ?     ?     ?.?%    ?.?%  +?.?%
deser            502    ?     ?     ?     ?     ?.?%    ?.?%  +?.?%
redos            1333   ?     ?     ?     ?     ?.?%    ?.?%  +?.?%
intoverflow      190    ?     ?     ?     ?     ?.?%    ?.?%  +?.?%
infodisclosure   200    ?     ?     ?     ?     ?.?%    ?.?%  +?.?%
----------------------------------------------------------------------
OVERALL                 ?     ?     ?     ?     ?.?%    ?.?%  +?.?%
```

---

## Scoring Script

After running `aud full --offline` on the benchmark directory, score with:

```bash
/mnt/c/Users/santa/Desktop/TheAuditorV2/.venv/Scripts/python.exe -c '
import sqlite3, os, re
from collections import defaultdict
from pathlib import Path

BENCH = Path(r"C:\Users\santa\Desktop\open_tests\gorustbash_benchmark\rust")
APPS = BENCH / "apps"
DB = BENCH / ".pf" / "repo_index.db"
CSV_FILE = BENCH / "expectedresults-0.1.csv"

if not DB.exists():
    print("ERROR: Database not found at %s" % DB)
    print("Run your SAST tool on this directory first to generate the database.")
    import sys; sys.exit(1)

# --- Parse CSV answer key (matches Java/Go format) ---
gt = []
with open(CSV_FILE, "r", encoding="utf-8") as f:
    for line in f:
        line = line.strip()
        if line.startswith("#") or not line:
            continue
        parts = line.split(",")
        gt.append({
            "key": parts[0],
            "category": parts[1],
            "vulnerable": parts[2].lower() == "true",
            "cwe": int(parts[3]),
        })

print("Answer key: %d test cases" % len(gt))

# --- Parse vuln-code-snippet annotations from source ---
snippets = {}
pat_s = re.compile(r"vuln-code-snippet\s+start\s+(\S+)")
pat_e = re.compile(r"vuln-code-snippet\s+end\s+(\S+)")

for scan_dir in [str(APPS), str(BENCH / "testcode")]:
  for root, dirs, files in os.walk(scan_dir):
    dirs[:] = [d for d in dirs if d != "target" and d != ".git"]
    for fn in files:
        if not fn.endswith(".rs"):
            continue
        fp = Path(root) / fn
        try:
            lines = open(fp, "r", encoding="utf-8", errors="replace").readlines()
        except:
            continue
        rel = str(fp.relative_to(BENCH)).replace("\\", "/")
        opens = {}
        for i, ln in enumerate(lines, 1):
            m = pat_s.search(ln)
            if m:
                opens[m.group(1)] = i
            m = pat_e.search(ln)
            if m:
                k = m.group(1)
                if k in opens:
                    snippets[k] = {"file": rel, "s": opens.pop(k), "e": i}

print("Annotations: %d snippets found" % len(snippets))

# Match ground truth keys to annotation line ranges
for tc in gt:
    sn = snippets.get(tc["key"])
    if sn:
        tc["file"] = sn["file"]
        tc["start_line"] = sn["s"]
        tc["end_line"] = sn["e"]
    else:
        tc["file"] = None
        tc["start_line"] = 0
        tc["end_line"] = 0

missing = [tc["key"] for tc in gt if tc["file"] is None]
if missing:
    print("WARNING: %d keys without annotations: %s" % (len(missing), ", ".join(missing[:5])))

# --- Rule name -> benchmark category (verified from rule source code) ---
RULE_MAP = {
    # Track A taint (rust_injection_analyze.py)
    "rust-command-injection-taint": "cmdi",
    "rust-sql-injection-taint": "sqli",
    "rust-path-traversal-taint": "pathtraver",
    "rust-ssrf-taint": "ssrf",
    # Track B structural (rust_injection_analyze.py)
    "rust-command-injection": "cmdi",
    "rust-sql-injection-format": "sqli",
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
    # Supply chain (supply_chain.py)
    "rust-weak-crypto-dependency": "crypto",
    "rust-deprecated-dependency": "crypto",
    # Hardcoded secrets
    "hardcoded-credential": "infodisclosure",
    "hardcoded-secret": "infodisclosure",
}
SINK_MAP = {
    "SQL Injection": "sqli", "Command Injection": "cmdi",
    "Path Traversal": "pathtraver", "SSRF": "ssrf",
    "Server-Side Request Forgery": "ssrf",
    "Cross-Site Scripting": "xss", "Cross-Site Scripting (XSS)": "xss",
    "Weak Cryptography": "crypto", "Memory Safety": "memsafety",
    "Insecure Deserialization": "deser",
}
NOISE = {"deadcode-function", "api-missing-auth", "missing-middleware"}

# --- Load findings from DB ---
conn = sqlite3.connect(str(DB))
c = conn.cursor()
det = defaultdict(set)

# Track 1: pattern_findings (rule results)
c.execute("SELECT file, line, rule FROM pattern_findings")
for f, ln, r in c.fetchall():
    if r not in NOISE:
        cat = RULE_MAP.get(r)
        if cat:
            det[f.replace("\\", "/")].add((ln, cat))

# Track 2: resolved_flow_audit (IFDS-confirmed VULNERABLE only)
c.execute("SELECT sink_file, sink_line, vulnerability_type FROM resolved_flow_audit WHERE status = ?", ("VULNERABLE",))
for f, ln, vt in c.fetchall():
    cat = SINK_MAP.get(vt)
    if cat:
        det[f.replace("\\", "/")].add((ln, cat))

# NOTE: taint_sinks deliberately EXCLUDED from scoring.
# Raw discovery table has no TP/FP discrimination -- causes FPR=100%.
# Java benchmark made the same decision (fakejavabench.md line 213).

conn.close()

# --- Score ---
cats = sorted(set(tc["category"] for tc in gt))
CWE_MAP = {}
for tc in gt:
    CWE_MAP[tc["category"]] = tc.get("cwe", 0)

print()
print("%-16s %-6s %-5s %-5s %-5s %-5s %7s %7s %7s" % (
    "Category", "CWE", "TP", "FP", "FN", "TN", "TPR", "FPR", "Score"))
print("-" * 75)

ttp = tfp = tfn = ttn = 0
for cat in cats:
    tests = [tc for tc in gt if tc["category"] == cat and tc["file"]]
    tp = fp = fn = tn = 0
    for tc in tests:
        file_findings = det.get(tc["file"], set())
        detected = any(
            tc["start_line"] <= ln <= tc["end_line"] and fc == cat
            for ln, fc in file_findings
        )
        if tc["vulnerable"]:
            if detected: tp += 1
            else: fn += 1
        else:
            if detected: fp += 1
            else: tn += 1
    tr = tp + fn; ts = fp + tn
    tpr = tp / tr if tr else 0; fpr = fp / ts if ts else 0
    ttp += tp; tfp += fp; tfn += fn; ttn += tn
    cwe = CWE_MAP.get(cat, 0)
    print("%-16s %-6d %-5d %-5d %-5d %-5d %6.1f%% %6.1f%% %+6.1f%%" % (
        cat, cwe, tp, fp, fn, tn, tpr * 100, fpr * 100, (tpr - fpr) * 100))

otpr = ttp / (ttp + tfn) if (ttp + tfn) else 0
ofpr = tfp / (tfp + ttn) if (tfp + ttn) else 0
print("-" * 75)
print("%-16s %-6s %-5d %-5d %-5d %-5d %6.1f%% %6.1f%% %+6.1f%%" % (
    "OVERALL", "", ttp, tfp, tfn, ttn, otpr * 100, ofpr * 100, (otpr - ofpr) * 100))
'
```

---

## Root Cause Analysis Template

For every FN (missed detection) and FP (false alarm), document:

| Test Case Key | Expected | Actual | Pipeline Stage | Root Cause |
|---------------|----------|--------|----------------|------------|
| sqliXxx | TP | FN | ? | ? |

Pipeline stages (from correctness_sop.md):
1. AST Extractor (rust_impl.py)
2. Post-Processor (indexer/extractors/rust.py)
3. Graph Strategy (graph/strategies/rust_web.py)
4. Taint Discoverer
5. Taint Patterns (taint/patterns/rust.py)
6. Flow Resolution (IFDS)
7. Rules (rules/rust/)

---

## Next Steps

1. Run `aud full --offline` on benchmark directory
2. Run scoring script to establish baseline
3. Root cause analysis for every FN and FP
4. Add safe variants (TN) for categories with 100/0 balance
5. Add edge case test cases (Phase 5 from plan)
6. Re-score and iterate
