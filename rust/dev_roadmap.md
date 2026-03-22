# Rust SAST Benchmark — Development Roadmap

> **Version:** 0.3.2 (2026-03-23)
> **Status:** v0.3.2 shipped. Phase 1: 12 misclassification fixes. Phase 2: testcode/ hint removal. Phase 2.5: apps/ hint purge. Phase 3: XSS rebalance (31/69 → 50/50, +6 TPs). SARIF scoring consolidated. CSV is sole ground truth. 268 test cases, all categories within 40-60 balance.
> **Gold standard reference:** OWASP BenchmarkJava (2,740 test cases, 11 CWEs, 52/48 TP/TN, +100% achieved)

---

## Current State (verified)

### What's built and verified correct
- **98 test case annotations** across 20 `.rs` files in 7 apps — 30/30 spot-checked, all correct
- **98 ground truth entries** (YAML) — 1:1 match with annotations, zero orphans in either direction
- **Scoring script** (Python, inline in rust_benchmark.md) — annotation-based, parses source at runtime
- **8 apps** copied from original Rust project, originals untouched
- **App catalog** — architecture, data flow, security posture documented for all 8 apps
- **Rule coverage gap analysis** — 8 gaps verified by reading engine rule source code
- **Pattern quality** — `taint/patterns/rust.py` audited clean (no defects DC-1 through DC-11)

### What's structurally wrong (vs Java gold standard)

| Problem | Impact | Severity |
|---------|--------|----------|
| **TN deficit** — 79/21 TP/TN ratio (Java: 52/48) | FPR unmeasurable for 9 of 13 categories | CRITICAL |
| **No isomorphic safe variants** — Safe code looks obviously different from vulnerable code | FP testing is trivial, not adversarial | HIGH |
| **Scale** — 97 test cases (Java: 2,740) | Not statistically significant per category | HIGH |
| **Annotation fragility** — Test identity in mutable source comments | Silent test case loss on refactor | MEDIUM |
| **No CSV answer key** — YAML + annotations require dual sync | Maintenance burden, audit complexity | MEDIUM |
| **6 CWE categories have no engine rules** — 21 test cases will always FN | Score ceiling limited by engine, not benchmark | KNOWN LIMITATION |
| **Scoring script untested** — Never run against real DB | May have bugs | LOW (fixable in M1) |

### Category health

| Category | CWE | TP | TN | Rule? | Status |
|----------|-----|----|----|-------|--------|
| sqli | 89 | 20 | 15 | YES | Healthy — 57/43 balance, taint+structural rule |
| memsafety | 119 | 7 | 4 | YES | Adequate — 64/36 balance, 3 structural rules |
| cmdi | 78 | 11 | 0 | YES | **Needs TN** — rule exists, no safe test cases |
| pathtraver | 22 | 11 | 1 | YES | **Needs TN** — rule exists, 1 safe test case |
| ssrf | 918 | 7 | 0 | YES | **Needs TN** — rule exists, no safe test cases |
| crypto | 327 | 5 | 0 | PARTIAL | **Needs TN + code-level rule** — dep-level only |
| weakrand | 330 | 3 | 0 | NO | **Needs TN + rule** |
| xss | 79 | 2 | 0 | NO | **Needs TN + rule** |
| infodisclosure | 200+ | 5 | 0 | NO | **Needs TN + rule** |
| deser | 502 | 2 | 0 | NO | **Needs TN + rule** |
| intoverflow | 190 | 2 | 0 | PARTIAL | **Needs TN** — rule exists for crypto context only |
| redos | 1333 | 1 | 0 | NO (.rs missing) | **Needs TN + .rs in target_extensions** |
| inputval | 20 | 1 | 0 | NO | **Needs TN + rule** |

---

## Milestone 1: Foundation Verification

**Goal:** First real scorecard. Prove the infrastructure works end-to-end.

**Scope:**
1. Run `aud full --offline` on `/open_tests/gorustbash_benchmark/rust/`
2. Run scoring script from `rust_benchmark.md`
3. Fix any scoring script bugs (YAML parsing, file path resolution, RULE_MAP gaps)
4. Document baseline scorecard in `rust_benchmark.md`
5. For every FN: identify which pipeline stage failed (extractor? graph? taint? rule? pattern?)
6. For every FP: identify why (annotation wrong? rule overfires? pattern too broad?)

**Files changed:**
- `rust_benchmark.md` — baseline scorecard + root-cause table

**Verification:**
- Scoring script runs without errors
- All 98 test cases evaluated (0 "missing annotation" warnings)
- Scorecard numbers are consistent (TP+FN = total TP in ground truth, FP+TN = total TN)

**Estimated effort:** Small — mostly running commands and documenting results.

**Status:** Scoring script fixed (v2: taint_sinks removed, RULE_MAP expanded from rule source, path normalization). Blocked on `aud full --offline` — human runs this.

**Prerequisite for:** All other milestones (need baseline to measure improvement)

---

## Milestone 2: TN Parity

**Goal:** Every category has safe test cases. FPR measurable across all 13 categories.

**Scope:**
For each category with 0 TN, create **isomorphic safe variants** — functions that use the SAME API as the vulnerable version but differ in ONE key aspect:

| Category | Vulnerable Pattern | Safe Variant (isomorphic) |
|----------|-------------------|--------------------------|
| cmdi | `Command::new(user_input)` | `Command::new("ls").arg("-la")` (hardcoded) |
| pathtraver | `fs::read_to_string(user_path)` | `canonicalize(path)` + `starts_with(base_dir)` check |
| ssrf | `reqwest::get(user_url)` | `Url::parse(url)` + domain allowlist check |
| crypto | `md5::compute(data)` | `sha2::Sha256::digest(data)` |
| weakrand | `format!("{}_{}", timestamp, user_id)` | `OsRng.gen::<[u8; 32]>()` + hex encode |
| xss | `RawHtml(format!("{}", user_input))` | `html_escape::encode_text(user_input)` |
| deser | `serde_json::from_slice(&raw_bytes)` | `serde_json::from_str::<TypedStruct>(input)` |
| redos | `Regex::new(&user_pattern)` | `Regex::new(r"^[a-zA-Z0-9]+$")` (hardcoded) |
| intoverflow | `a.wrapping_add(b)` | `a.checked_add(b).ok_or(Error)?` |
| infodisclosure | `eprintln!("{:?}", full_error)` | `HttpResponse::InternalServerError().body("Internal error")` |
| inputval | `amount: req.amount` (raw) | `if req.amount < 0 { return Err(...) }` then use |

**Rules for safe variants:**
1. Place in SAME file as the vulnerable version (or adjacent in same module)
2. Function name must be similar: `search_users_vulnerable` / `search_users_safe`
3. Same function signature where possible
4. The ONLY difference is the security-relevant line
5. Annotate with `vuln-code-snippet start/end` + `safe-line`
6. Add to ground truth YAML with `vulnerable: false`

**Target:** 50+ new TN test cases. Brings ratio from 79/21 to ~55/45.

**Files changed:**
- 10-15 `.rs` files (new safe functions added)
- `rust_ground_truth.yml` — 50+ new entries (file deleted in v0.3.1 — CSV is sole ground truth)
- `rust_benchmark.md` — updated inventory table

**Verification:**
- Every TN test case has a corresponding TP in the same category
- No category has 0 TN after this milestone
- Annotation count still matches ground truth count
- Re-run scoring script — FPR column now has real numbers

**Estimated effort:** Large — this is the biggest single milestone. 50+ new Rust functions to write, each must be correct.

**Prerequisite:** M1 (need baseline to know which categories need work)

---

## Milestone 3: Answer Key Migration

**Goal:** Standalone CSV answer key matching the Java benchmark format.

**Scope:**
1. Generate `expectedresults-0.1.csv` from ground truth YAML
2. Format: `test_name,category,vulnerable,cwe` (4 fields, same as Java)
3. Rewrite scoring script to read CSV instead of YAML
4. Keep YAML as documentation but CSV is the scoring authority
5. Keep annotation-based line range detection (still needed for file+line matching)

**Format:**
```csv
# test name,category,real vulnerability,CWE
sqliSearchUsersVulnerable,sqli,true,89
sqliGetUserByIdSafe,sqli,false,89
cmdiExecuteCommand,cmdi,true,78
...
```

**Files changed:**
- NEW: `expectedresults-0.1.csv`
- `rust_benchmark.md` — updated scoring script
- `rust_ground_truth.yml` — marked as "documentation, not scoring authority" (file deleted in v0.3.1)

**Verification:**
- CSV has exactly as many lines as ground truth YAML entries
- Scoring script produces identical results with CSV as with YAML
- CSV is parseable by any tool (Python csv module, Excel, etc.)

**Estimated effort:** Small — mechanical transformation.

---

## Milestone 4: Dedicated Test Case Files (optional)

**Goal:** Standalone test files matching the Java one-file-per-test pattern.

**Scope:**
- Create `testcode/` directory
- Each test case becomes its own `.rs` file with minimal boilerplate
- File naming: `BenchmarkTest_sqli_001.rs` (vulnerable), `BenchmarkTest_sqli_002.rs` (safe)
- Each file is a complete, compilable Rust function (no framework dependency needed)
- Keep `apps/` as the "real world" test suite
- Both `testcode/` and `apps/` contribute to scoring

**Why optional:** This is the highest effort milestone with diminishing returns. The Java benchmark uses this structure because Servlet test cases are naturally self-contained. Rust test cases often depend on framework extractors (actix, axum) which require external crate dependencies. Standalone files would need to simulate framework input, reducing realism.

**Alternative:** Keep apps/ as primary, but ensure every test case is clearly isolated within its file (one function = one test case, no overlapping annotations).

**Estimated effort:** Very large. Only pursue if M1-M3 and M5 are complete.

---

## Milestone 5: Scale to 300+

**Goal:** Statistically significant test case count per category.

**Scope:**
- Minimum 10 TP + 10 TN per category = 20 per category x 13 categories = 260 minimum
- Add pattern diversity:
  - **Framework variations:** Same vulnerability across actix-web, axum, rocket, warp
  - **Flow depth:** Direct (1-hop), indirect (2-hop), deep (3+ hop through async/closures/traits)
  - **Obfuscation:** Dead code branches, variable reassignment before use, conditional sanitization
  - **Real CVE patterns:** From OSSF CVE database (`/open_tests/ossf/CVEs/`)
- Each new test case must have both TP and TN variant

**Per-category expansion targets:**

| Category | Current | Target | New TP | New TN |
|----------|---------|--------|--------|--------|
| sqli | 35 | 50 | 5 | 10 |
| cmdi | 11 | 30 | 5 | 14 |
| pathtraver | 12 | 30 | 4 | 14 |
| ssrf | 7 | 20 | 3 | 10 |
| memsafety | 11 | 20 | 0 | 9 |
| crypto | 5 | 20 | 5 | 10 |
| weakrand | 3 | 20 | 7 | 10 |
| xss | 2 | 20 | 8 | 10 |
| infodisclosure | 5 | 20 | 5 | 10 |
| deser | 2 | 20 | 8 | 10 |
| intoverflow | 2 | 20 | 8 | 10 |
| redos | 1 | 10 | 4 | 5 |
| inputval | 1 | 10 | 4 | 5 |
| **TOTAL** | **97** | **290** | **66** | **127** |

**Estimated effort:** Very large — 190+ new test cases. Multiple work sessions.

---

## Milestone 6: CI & Automation

**Goal:** Automated validation, scoring, and regression detection.

**Scope:**
1. Script: validate annotations match ground truth/CSV (key consistency check)
2. Script: run scoring against DB and output scorecard
3. GitHub Actions workflow: run validation on every PR
4. Contribution template: require TP+TN pair for new test cases

**Files:**
- NEW: `scripts/validate.py` — key consistency check
- NEW: `scripts/score.py` — standalone scoring script (extracted from benchmark.md)
- NEW: `.github/workflows/benchmark.yml`
- NEW: `.github/PULL_REQUEST_TEMPLATE.md`

**Estimated effort:** Medium.

---

## Dependency Graph

```
M1 (Baseline)
 |
 +---> M2 (TN Parity) ---> M5 (Scale to 300+)
 |
 +---> M3 (CSV Migration)
 |
 +---> M6 (CI/Automation)
 |
 +---> M4 (Dedicated Files) -- optional, after M2+M5
```

M1 is the prerequisite for everything. M2, M3, M6 can run in parallel after M1. M5 depends on M2 (need the TN pattern established before scaling). M4 is optional and depends on M2+M5.

---

## Known Limitations (cannot be fixed by benchmark team)

These require engine work, not benchmark work. Documented in the rule coverage matrix in `rust_benchmark.md`.

| Gap | What's Missing | Owner |
|-----|---------------|-------|
| GAP-R1: XSS | No Rust XSS rule | Engine team |
| GAP-R2: Weak Random | No weak RNG rule | Engine team |
| GAP-R3: Deserialization | No Rust deser rule | Engine team |
| GAP-R4: ReDoS | `.rs` not in redos_analyze target_extensions | Engine team (LOW fix) |
| GAP-R5: Info Disclosure | No Rust info disclosure rule | Engine team |
| GAP-R6: Input Validation | No input validation rule | Engine team |
| GAP-R7: Weak Crypto (code-level) | Only dep-level detection | Engine team |
| GAP-R8: Integer Overflow (general) | Only crypto/financial context | Engine team |

The benchmark EXPOSES these gaps. That's the value. When engine rules are added, the benchmark score will improve — measuring exactly how much each rule contributes.

---

## Next Action

**Execute Milestone 1.** It's the smallest milestone with the highest information yield. Running `aud full --offline` and the scoring script will immediately reveal:
- Which categories the engine already handles for Rust
- Which test cases are detected (TP) vs missed (FN)
- Whether the scoring infrastructure works
- The real baseline score to improve from

Everything after M1 is informed by M1's results. Don't skip it.
