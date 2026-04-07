# Rust SAST Benchmark

## Methodology

Modeled after OWASP BenchmarkJava (the gold standard — 2,740 test cases, 100% achieved).

**Ground truth**: `expectedresults-0.5.1.csv` — CSV answer key (sole scoring authority). Matches OWASP Java/Python format.
**Scoring**: Youden's J (TPR - FPR) per CWE category. 0% = random guessing. +100% = perfect.

### Test Case Inventory (v0.5.1)

| Category | CWE | TP | TN | Total |
|----------|-----|----|----|-------|
| sqli | 89 | 5 | 11 | 16 |
| cmdi | 78 | 14 | 20 | 34 |
| pathtraver | 22 | 14 | 24 | 38 |
| ssrf | 918 | 18 | 20 | 38 |
| memsafety | 119 | 18 | 21 | 39 |
| crypto | 327* | 20 | 21 | 41 |
| weakrand | 330 | 22 | 23 | 45 |
| xss | 79 | 23 | 23 | 46 |
| infodisclosure | 200* | 22 | 25 | 47 |
| deserial | 502 | 23 | 24 | 47 |
| intoverflow | 190 | 23 | 24 | 47 |
| hardcodedcreds | 798 | 23 | 25 | 48 |
| redos | 1333 | 24 | 24 | 48 |
| inputval | 20 | 24 | 25 | 49 |
| race_condition | 362 | 25 | 25 | 50 |
| loginjection | 117 | 25 | 25 | 50 |
| securecookie | 614 | 25 | 25 | 50 |
| redirect | 601 | 25 | 25 | 50 |
| fileupload | 434 | 25 | 25 | 50 |
| tlsverify | 295 | 25 | 25 | 50 |
| authnfailure | 287 | 25 | 25 | 50 |
| csrf | 352 | 25 | 25 | 50 |
| authzfailure | 285 | 25 | 25 | 50 |
| ldapi | 90 | 25 | 25 | 50 |
| nosql | 943 | 25 | 25 | 50 |
| **TOTAL** | | **548** | **585** | **1,133** |

*crypto has entries with CWE-347 (JWT alg=none); infodisclosure has entries with CWE-200/209/532

**Testcode-only counts** (119 app entries moved to `vulnerable_apps/rust/` for separate scoring). Categories with heavy app contributions (sqli, cmdi, pathtraver, ssrf, memsafety) are below the 25/25 floor. FPR measurable for 100% of test cases.

### Anti-Target Leakage Rules (v0.5.1)

- Test filenames use opaque `benchmark_test_NNNNN.rs` naming. No category or CWE in filename.
- Test files contain zero comments. No CWE references, no vulnerability descriptions, no annotation markers.
- 1 file = 1 test case = 1 `pub fn handle()` function.
- Tools must rely on AST/dataflow analysis to classify test cases. Text-matching filenames or comments is not possible.
- Naming convention matches OWASP BenchmarkJava and the Go benchmark (`BenchmarkTestNNNNN`).

**Complexity tiers** (no tier label in source files — tiers are for benchmark design documentation only):
- T1 (Direct): req.param() → sink in ≤3 lines (~40% of new cases)
- T2 (Indirect): taint through 1 intermediate — struct field, format!, helper fn (~35%)
- T3 (Hard TN): dead-code branch / overwrite / Vec-remove / HashMap-key-mismatch / ignore-arg (~15% of new TN)
- T4 (Hard TP): incomplete validation bypass — partial char check, length-only, prefix-doesn't-prevent (~15% of new TP)

### Frameworks Covered

Testcode uses no framework (raw function signatures). Reference apps in `vulnerable_apps/rust/` cover actix-web, axum, Rocket, and Warp.

---

## App Catalog (moved to vulnerable_apps/rust/)

### 1. rust_taint_app (actix-web + sqlx/rusqlite)
**Purpose:** Intentional taint flow test app. Every handler is a taint source, downstream modules are sinks.
**Architecture:** `handlers.rs` -> `database.rs` / `commands.rs` / `files.rs` / `network.rs` / `memory_ops.rs`
**Security posture:** ~50% vulnerable, ~50% safe by design. Both patterns exist for SQL, file ops.
**Test cases:** 23 (Phase 1)

### 2. deepflow-rust (actix-web)
**Purpose:** Deep taint flow complexity testing. Pipeline depth, async boundaries, closure captures, iterator chains, trait dispatch.
**Architecture:** `handlers.rs` -> `pipeline.rs` / `async_flow.rs` / `closures.rs` / `iterators.rs` / `traits.rs` / `advanced.rs` -> `sinks.rs`
**Security posture:** Nearly ALL vulnerable by design. Tests flow tracing, not pattern detection. Many sinks are simulated (println! not real SQL).
**Scoring:** EXCLUDED from ground truth — value is taint stress testing, not TP/TN benchmark scoring.

### 3. rust_backend (actix-web)
**Purpose:** Backend with intentional vulnerability categories (SQLi, ReDoS, weak crypto, race condition, integer overflow, deserialization, memory corruption).
**Architecture:** `handlers.rs` -> `patterns.rs` + `memory_ops.rs`
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
| deserial | 502 | NONE | - | **GAP** (patterns exist, no rule) |
| redos | 1333 | `security/redos_analyze.py` (no .rs) | - | **GAP** (LOW fix: add .rs) |
| intoverflow | 190 | `rust/integer_safety.py` | structural | **PARTIAL** (crypto/financial only) |
| infodisclosure | 200+ | NONE | - | **GAP** |
| inputval | 20 | `security/input_validation_analyze.py` | structural | FULL |
| hardcodedcreds | 798 | NONE | - | **GAP** |
| race_condition | 362 | NONE | - | **GAP** |
| loginjection | 117 | NONE | - | **GAP** |
| securecookie | 614 | NONE | - | **GAP** |
| redirect | 601 | NONE | - | **GAP** |
| fileupload | 434 | NONE | - | **GAP** |
| tlsverify | 295 | NONE | - | **GAP** |

**218 of 491 test cases (44%) are in gap categories** (xss=24, weakrand=24, infodisclosure=24, deserial=24, redos=24, hardcodedcreds=22, race_condition=20, loginjection=20, securecookie=20, redirect=20, fileupload=20, tlsverify=20) -- expected to show as FN in baseline scoring.

---

## Current Scorecard

See [baseline_theauditor_tool_score.md](baseline_theauditor_tool_score.md) for full scorecard and version history.

Score via the CWE-based SARIF pipeline (see [SCORING.md](SCORING.md)):
```bash
python ../scripts/convert_theauditor.py .pf/repo_index.db
python ../scripts/score_sarif.py theauditor.sarif expectedresults-0.5.1.csv
```

---

## Scoring

See [SCORING.md](SCORING.md) for full tool-agnostic scoring instructions (SARIF-based, works with any SAST tool).

**Matching is CWE-based** — SARIF ruleId must be a CWE number. No hand-maintained RULE_MAP.

### Quick Start (any tool)

```bash
# Export SARIF from your tool, then:
python ../scripts/score_sarif.py results.sarif expectedresults-0.5.1.csv
```

### TheAuditor (database-first path)

```bash
aud full --offline
python ../scripts/convert_theauditor.py .pf/repo_index.db
python ../scripts/score_sarif.py theauditor.sarif expectedresults-0.5.1.csv
```

---

## Root Cause Analysis Template

For every FN (missed detection) and FP (false alarm), document:

| Test Case Key | Expected | Actual | Pipeline Stage | Root Cause |
|---------------|----------|--------|----------------|------------|
| sqliXxx | TP | FN | ? | ? |

Pipeline stages:
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
