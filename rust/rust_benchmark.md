# Rust SAST Benchmark

## Methodology

Modeled after OWASP BenchmarkJava (the gold standard — 2,740 test cases, 100% achieved).

**Ground truth**: `expectedresults-0.3.2.csv` — CSV answer key (sole scoring authority). Matches OWASP Java/Python format.
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
| deser | 502 | 6 | 6 | 12 | 50/50 |
| intoverflow | 190 | 5 | 7 | 12 | 42/58 |
| redos | 1333 | 5 | 5 | 10 | 50/50 |
| inputval | 20 | 4 | 6 | 10 | 40/60 |
| **TOTAL** | | **118** | **144** | **262** | **45/55** |

**All 13 categories have TP AND TN.** Every category can measure both TPR and FPR. TP/TN ratio: 45/55 (Java gold standard: 52/48). FPR measurable for 100% of test cases.

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
| deser | 502 | NONE | - | **GAP** (patterns exist, no rule) |
| redos | 1333 | `security/redos_analyze.py` (no .rs) | - | **GAP** (LOW fix: add .rs) |
| intoverflow | 190 | `rust/integer_safety.py` | structural | **PARTIAL** (crypto/financial only) |
| infodisclosure | 200+ | NONE | - | **GAP** |
| inputval | 20 | `security/input_validation_analyze.py` | structural | FULL |

**22 of 262 test cases (8%) are in gap categories** -- expected to show as FN in baseline scoring.

---

## Current Scorecard

See [baseline_theauditor_tool_score.md](baseline_theauditor_tool_score.md) for full scorecard and version history.

Score via the standardized SARIF pipeline (see [SCORING.md](SCORING.md)):
```bash
python ../scripts/convert_theauditor.py .pf/repo_index.db --language rust --benchmark-dir .
python ../scripts/score_sarif.py theauditor.sarif expectedresults-0.3.2.csv \
    --annotations-dir apps --annotations-dir testcode
```

---

## Scoring

See [SCORING.md](SCORING.md) for full tool-agnostic scoring instructions (SARIF-based, works with any SAST tool).

### Quick Start (any tool)

```bash
# Export SARIF from your tool, then:
python ../scripts/score_sarif.py results.sarif expectedresults-0.3.2.csv \
    --annotations-dir apps --annotations-dir testcode
```

### TheAuditor (database-first path)

```bash
aud full --offline
python ../scripts/convert_theauditor.py .pf/repo_index.db \
    --language rust --benchmark-dir . > theauditor.sarif
python ../scripts/score_sarif.py theauditor.sarif expectedresults-0.3.2.csv \
    --annotations-dir apps --annotations-dir testcode
```

### Legacy Embedded Script — DELETED

The embedded TheAuditor-specific scoring script was removed in v0.3.2. All scoring now goes through the standard SARIF pipeline:

1. `python ../scripts/convert_theauditor.py .pf/repo_index.db --language rust --benchmark-dir .`
2. `python ../scripts/score_sarif.py theauditor.sarif expectedresults-0.3.2.csv --annotations-dir apps --annotations-dir testcode`

See `SCORING.md` for full documentation.

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
