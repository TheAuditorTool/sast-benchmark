# Rust SAST Benchmark — Change Log

Every change to benchmark files documented here with rationale.

---

## 2026-04-07 — v0.4.0: Major Expansion (268 → 491 test cases, 13 → 20 CWEs)

### Context
OWASP Foundation submission requires comprehensive CWE coverage and statistical significance per category. v0.3.2 had 268 tests across 13 CWEs but several categories were below the 10 TP / 10 TN minimum threshold. This release nearly doubles the benchmark and adds 7 new CWE categories present in Go/PHP/Ruby benchmarks.

### Pre-Flight Due Diligence
- All 7 new CWE numbers verified against MITRE CWE database (cwe.mitre.org)
- Cross-benchmark consistency audit against Go, PHP, Ruby, and Java gold standard
- TP/TN classification corrections applied from security audit (DashMap entry() API, Rocket add_private, tracing JSON subscriber, zip slip ZipFile::name)

### Phase A: Reclassifications (2 entries)

| Entry | Old | New | Rationale |
|-------|-----|-----|-----------|
| infodisclosureCalorieJwtSecret | infodisclosure,true,798 | hardcodedcreds,true,798 | Hardcoded JWT secret is textbook CWE-798 |
| infodisclosurePaymentsHardcodedKey | infodisclosure,true,798 | hardcodedcreds,true,798 | Hardcoded Stripe key is textbook CWE-798 |

### Phase B: Expand Existing 13 CWEs (+83 test files)

All existing categories brought to minimum 12 TP / 12 TN:

| Category | Before | After | New Files |
|----------|--------|-------|-----------|
| ssrf | 9/13=22 | 12/13=25 | +3 TP |
| xss | 11/11=22 | 12/12=24 | +1 TP, +1 TN |
| memsafety | 8/12=20 | 12/12=24 | +4 TP |
| crypto | 9/11=20 | 12/12=24 | +3 TP, +1 TN |
| weakrand | 7/9=16 | 12/12=24 | +5 TP, +3 TN |
| infodisclosure | 6/8=14* | 12/12=24 | +6 TP, +4 TN |
| deser | 6/6=12 | 12/12=24 | +6 TP, +6 TN |
| intoverflow | 5/7=12 | 12/12=24 | +7 TP, +5 TN |
| redos | 5/5=10 | 12/12=24 | +7 TP, +7 TN |
| inputval | 4/6=10 | 12/12=24 | +8 TP, +6 TN |

*After reclassifying 2 CWE-798 entries to hardcodedcreds

### Phase C: 7 New CWE Categories (+140 test files)

| Category | CWE | TP | TN | Total | Justification |
|----------|-----|----|----|-------|---------------|
| hardcodedcreds | 798 | 12* | 10 | 22 | OWASP A07, present in Go/PHP/Ruby/Bash |
| race_condition | 362 | 10 | 10 | 20 | Present in Go/Bash, Rust-specific logic races |
| loginjection | 117 | 10 | 10 | 20 | OWASP A09, present in Go/Ruby |
| securecookie | 614 | 10 | 10 | 20 | Java gold standard, Go/PHP/Ruby |
| redirect | 601 | 10 | 10 | 20 | OWASP A01, Go/PHP/Ruby |
| fileupload | 434 | 10 | 10 | 20 | OWASP A04, Go/PHP/Ruby |
| tlsverify | 295 | 10 | 10 | 20 | Present in Go/Bash |

*Includes 2 reclassified from infodisclosure + 10 new

### Phase D: Infrastructure Updates

- `expectedresults-0.4.0.csv` — 491 entries (new file)
- `scripts/validate_rust.py` — VALID_CWES +6, VALID_CATEGORIES +7, CSV ref updated
- `rust/rust_benchmark.md` — inventory table, category count (13→20)
- `rust/SCORING.md` — CSV filename reference
- `rust/dev_roadmap.md` — milestone update
- `README.md` — Rust section updated (268→491, 13→20)

### Final Inventory

| Category | CWE | TP | TN | Total | Balance |
|----------|-----|----|----|-------|---------|
| sqli | 89 | 23 | 27 | 50 | 46/54 |
| cmdi | 78 | 14 | 16 | 30 | 47/53 |
| pathtraver | 22 | 14 | 14 | 28 | 50/50 |
| ssrf | 918 | 12 | 13 | 25 | 48/52 |
| xss | 79 | 12 | 12 | 24 | 50/50 |
| memsafety | 119 | 12 | 12 | 24 | 50/50 |
| crypto | 327* | 12 | 12 | 24 | 50/50 |
| weakrand | 330 | 12 | 12 | 24 | 50/50 |
| infodisclosure | 200* | 12 | 12 | 24 | 50/50 |
| deser | 502 | 12 | 12 | 24 | 50/50 |
| intoverflow | 190 | 12 | 12 | 24 | 50/50 |
| redos | 1333 | 12 | 12 | 24 | 50/50 |
| inputval | 20 | 12 | 12 | 24 | 50/50 |
| hardcodedcreds | 798 | 12 | 10 | 22 | 55/45 |
| race_condition | 362 | 10 | 10 | 20 | 50/50 |
| loginjection | 117 | 10 | 10 | 20 | 50/50 |
| securecookie | 614 | 10 | 10 | 20 | 50/50 |
| redirect | 601 | 10 | 10 | 20 | 50/50 |
| fileupload | 434 | 10 | 10 | 20 | 50/50 |
| tlsverify | 295 | 10 | 10 | 20 | 50/50 |
| **TOTAL** | | **243** | **248** | **491** | **49/51** |

---

## 2026-03-23 — v0.3.2: XSS Category Rebalance

### Context
Due diligence audit found XSS was the worst-balanced category in the entire benchmark: 5 TP vs 11 TN (31/69). OWASP standard is 50/50. Every other Rust category was within 40-60 range.

### Changes
Added 6 new XSS True Positive test cases, each a genuinely distinct CWE-79 variant:

| File | Key | Pattern | What It Tests |
|------|-----|---------|---------------|
| xss_013.rs | testcodeXss013 | Event handler injection (onclick) | Nested context awareness |
| xss_014.rs | testcodeXss014 | javascript: protocol in href | URL scheme validation |
| xss_015.rs | testcodeXss015 | Stored XSS via in-memory HashMap | Cross-function taint tracking |
| xss_016.rs | testcodeXss016 | HTML comment injection | Comment context breakout |
| xss_017.rs | testcodeXss017 | data: URI in iframe src | URI scheme + iframe context |
| xss_018.rs | testcodeXss018 | Error path reflection | Error branch taint flow |

**Result:** XSS now 11 TP + 11 TN = 50/50. Total benchmark: 268 test cases (123 TP, 145 TN).

---

## 2026-03-22 — v0.3.2: Apps Hint Purge (OWASP Gold Standard)

### Context
Phase 2 (v0.3.1) cleaned all 143 testcode/ files of classification hints. However, the 119 apps/ test cases across 7 reference applications still contained ~313 classification markers that violated the OWASP Benchmark principle: "CSV is the sole ground truth." This release purges all remaining hints from apps/.

### Changes

**Annotation markers neutralized (119 occurrences across 24 files):**
- All `vuln-code-snippet vuln-line` → `vuln-code-snippet target-line` (78)
- All `vuln-code-snippet safe-line` → `vuln-code-snippet target-line` (41)

**42 annotation keys renamed to remove Vulnerable/Safe suffixes:**
- 7 keys: stripped "Vulnerable" → base name (e.g., `sqliRocketGetUserVulnerable` → `sqliRocketGetUser`)
- 18 keys: stripped "Safe" → base name, no collision (e.g., `sqliCalorieCreateUserSafe` → `sqliCalorieCreateUser`)
- 17 keys: stripped "Safe" → collision with existing TP, appended "2" (e.g., `cmdiExecuteCommandSafe` → `cmdiExecuteCommand2`)

**~191 VULNERABILITY comment markers removed from 27+ .rs files:**
- `// VULNERABILITY:` and `/// VULNERABILITY:` prefixes stripped from all section headers, doc comments, and inline comments
- Section headers rewritten to neutral descriptions (e.g., "VULNERABILITY: SQL Injection via raw query string" → "SQL query construction")

**Module-level doc comments neutralized (5 files):**
- Removed "intentionally vulnerable", "INTENTIONAL VULNERABILITIES:", "dangerous operations that should be flagged" from `//!` comments

**Cargo.toml hint comments cleaned (3 files):**
- Removed "intentionally vulnerable for testing" and "intentionally using weak" comments

**CSV updated:** All 42 key renames applied. Header bumped to v0.3.2.

### What was NOT changed
- Function names (e.g., `unsafe_deserialize`, `greet_vulnerable`) — source code, not metadata. Renaming breaks compilation.
- VulnResponse.vulnerability field — application logic.
- Route paths, taint sink comments — describe code behavior, not TP/TN classification.

---

## 2026-03-22 — SARIF Scoring Support

### Tool-Agnostic Scoring
- Created `rust/SCORING.md` — mirrors Go's SCORING.md with Rust-specific annotation-based matching
- Extended `scripts/score_sarif.py` with `--annotations-dir` flag for annotation-based matching (Rust/Bash)
- Extended `scripts/convert_theauditor.py` with Rust RULE_MAP (40+ rules) and SINK_MAP entries
- Updated `rust_benchmark.md` to reference SCORING.md and show both SARIF and legacy TheAuditor paths
- Any SAST tool that outputs SARIF 2.1.0 can now be scored against the Rust benchmark

---

## 2026-03-22 — v0.3.1: Misclassification Fixes (OWASP Submission Response)

### Context
OWASP Foundation reviewed the v0.3 submission and provided feedback. Independent audit of all 262 test cases found 12 misclassified test cases across 6 categories. This release fixes all 12.

### MC-1 through MC-5: ReDoS — Wrong Regex Crate (5 test cases)

**Problem:** All 5 ReDoS TP test cases used `regex::Regex` (Rust's standard regex crate). This crate uses a linear-time NFA/DFA engine that is provably immune to catastrophic backtracking. ReDoS is impossible regardless of pattern complexity. These were false TPs — the code was not actually vulnerable.

**Fix:** Changed import to `fancy_regex::Regex` in all 5 files. `fancy-regex` uses a backtracking engine that supports lookaheads and backreferences, which breaks the linear-time guarantee and allows catastrophic backtracking. Added `.unwrap_or(false)` to `is_match()` calls (fancy-regex returns `Result<bool>`, not `bool`). Added `fancy-regex = "0.11"` to `apps/rust_backend/Cargo.toml` (was missing entirely — `regex` was also absent).

**Files changed:**
- `testcode/redos_001_vulnerable.rs` — `use regex::Regex` → `use fancy_regex::Regex`
- `testcode/redos_002_vulnerable.rs` — same
- `testcode/redos_003_vulnerable.rs` — same
- `testcode/redos_006_safe.rs` — same + annotation key renamed (see MC-5 below)
- `apps/rust_backend/src/vulnerable.rs` (redosBackendRegexDos function) — same
- `apps/rust_backend/Cargo.toml` — added `fancy-regex = "0.11"`

**MC-5 additionally:** `testcodeRedos006Safe` renamed to `testcodeRedos006Vulnerable` in source, CSV, and YAML. The file demonstrates a post-execution timeout that does not prevent ReDoS — it is a TP, but was named "Safe" misleadingly. CSV classification was already `true` (correct); only the key name was wrong.

### MC-6: Deserialization — Wrong API (1 test case)

**Problem:** `testcode/deser_002_vulnerable.rs` claimed to test a YAML bomb (billion laughs) attack via `serde_yaml::from_str`, but the actual code called `serde_json::from_str`. JSON has no alias/anchor expansion mechanism — YAML bombs are impossible with a JSON parser. The TP claim was based on a nonexistent attack vector.

**Fix:** Replaced the `serde_json` stub with actual `serde_yaml::from_str(&yaml_input)`. Return type changed to `serde_yaml::Value`. The file now genuinely parses YAML without size limits, making YAML bomb attacks possible.

**File changed:** `testcode/deser_002_vulnerable.rs`

### MC-7: Deserialization — False Safe (1 test case)

**Problem:** `testcode/deser_008_safe.rs` used a Content-Type header check as its "safe" mitigation. Content-Type is a client-supplied HTTP request header — an attacker sets it to `application/json` trivially. This check provides zero security against insecure deserialization.

**Fix:** Reclassified as TP. Annotation key renamed from `testcodeDeser008Safe` to `testcodeDeser008Vulnerable`. CSV changed from `false` to `true`. Doc comment rewritten to explain why Content-Type is not a security boundary.

**Files changed:** `testcode/deser_008_safe.rs`, `expectedresults-0.1.csv`, `rust_ground_truth.yml`

**Impact:** deser category balance improved from 5 TP / 7 TN to 6 TP / 6 TN (50/50).

### MC-8: SSRF — Timeout Is Not Mitigation (1 test case)

**Problem:** `testcode/ssrf_005_safe.rs` used a 5-second request timeout as its SSRF mitigation. Internal services (e.g., AWS metadata at 169.254.169.254) respond in milliseconds. A timeout does not prevent SSRF — it only limits slow-loris scenarios.

**Fix:** Replaced the timeout with a domain allowlist: parse URL, extract hostname, check against `["api.example.com", "cdn.example.com", "hooks.slack.com"]`. Domain allowlisting is a genuine SSRF mitigation. TN classification unchanged.

**File changed:** `testcode/ssrf_005_safe.rs`

### MC-9: SSRF — HTTPS-Only Is Not Mitigation (1 test case)

**Problem:** `testcode/ssrf_006_safe.rs` required HTTPS scheme only. `https://169.254.169.254/latest/meta-data/` is a valid HTTPS SSRF target. HTTPS-only does not prevent SSRF to internal endpoints that support TLS.

**Fix:** Replaced HTTPS-only check with private IP rejection: blocks `127.x`, `10.x`, `172.16-31.x`, `192.168.x`, `169.254.x`, `0.x`, `::1`, `localhost`. This is a genuine SSRF mitigation. TN classification unchanged.

**File changed:** `testcode/ssrf_006_safe.rs`

### MC-10: Memory Safety — Bounds-Checked Unsafe Labeled as Vulnerable (1 test case)

**Problem:** `memsafetyBackendMemoryCorruption` in `apps/rust_backend/src/vulnerable.rs` had `if offset < data.len() { Some(*data.get_unchecked(offset)) }`. The bounds check before `get_unchecked` made the code actually safe — out-of-bounds access was impossible. The TP classification was wrong.

**Fix:** Removed the `if offset < data.len()` guard. `get_unchecked(offset)` is now called directly without bounds checking, making out-of-bounds read genuinely possible when `offset >= data.len()`. TP classification is now correct.

**File changed:** `apps/rust_backend/src/vulnerable.rs`

### MC-11 & MC-12: Weak Random — XOR Is Not CSPRNG (2 test cases)

**Problem:** `weakrandJobqueueTokenSafe` and `weakrandJobqueueApiKeySafe` in `apps/rust_jobqueue/crates/jobqueue-api/src/auth.rs` XOR'd user_id/name bytes with timestamp bytes. Both inputs are attacker-observable or predictable. XOR of deterministic inputs produces deterministic output — this is not cryptographically secure randomness. The TN classification was wrong.

**Fix:** Replaced both XOR loops with `rand::rngs::OsRng.gen()` — the OS-level CSPRNG. Removed all timestamp and user_id/name mixing from the random bytes. Token/key entropy now comes purely from the OS entropy pool. TN classification is now correct.

**File changed:** `apps/rust_jobqueue/crates/jobqueue-api/src/auth.rs`

### Inventory After v0.3.1

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

### Phase 2: Hint Removal (OWASP Gold Standard Compliance)

Stripped all vulnerability classification hints from source code. The CSV is now the sole oracle — matching OWASP Java/Python benchmark design where source code contains zero hints about whether a test case is vulnerable or safe.

**testcode/ (143 files):**
- Rewrote `//!` doc-comment line 1 from `<Category> True Positive/Negative — CWE-XX` to neutral `CWE-XX: <description>`
- Removed all `//!` lines containing "True Positive", "True Negative", or "Isomorphic to"
- Removed all `// VULNERABLE:` (37 instances) and `// SAFE:` (97 instances) inline comments
- Unified `vuln-line`/`safe-line` annotation markers to neutral `target-line`
- Stripped `Vulnerable`/`Safe` suffix from all annotation keys (143 keys in source, CSV, and YAML)
- Renamed all files: `cmdi_001_vulnerable.rs` → `cmdi_001.rs`, `cmdi_002_safe.rs` → `cmdi_002.rs`

**apps/ (12 files):**
- Removed all `// VULNERABLE:`, `// SAFE:`, `// VULN:` inline comments (~109 instances)
- Function names (`_vulnerable`/`_safe`) and annotation keys unchanged (call site dependencies)

**Infrastructure:**
- Deleted `rust_ground_truth.yml` — technical debt. CSV is sole source of truth, matching Go benchmark and OWASP Java/Python format.
- Updated `rust_benchmark.md` to remove YAML reference.

### Validation
```
validate_rust.py: PASS — 262 CSV, 262 annotations, 118 TP, 144 TN, 0 errors
grep "True Positive|True Negative" testcode/ — 0 results
grep "VULNERABLE:|SAFE:|VULN:" testcode/ apps/ — 0 results
ls testcode/*_vulnerable* testcode/*_safe* — 0 files
```

---

## 2026-03-19 — Initial Setup (Phase 1)

### Infrastructure Created
- Copied 8 Rust apps from `C:\Users\santa\Desktop\rust\` to `apps/`
- Apps: rust_taint_app, deepflow-rust, rust_backend, rocket_test, warp_test, rust_calorie_app, rust_jobqueue, anarchy_commerce
- Total: 103 .rs files across 4 frameworks (actix-web, axum, rocket, warp)

### Ground Truth Created
- `rust_ground_truth.yml` v0.1 — initial test cases from existing code inventory
- 12 CWE categories targeted: sqli, cmdi, pathtraver, ssrf, xss, crypto, weakrand, memsafety, deser, redos, intoverflow, infodisclosure

### Scoring Script Created
- `rust_benchmark.md` — scoring script modeled after OWASP Java benchmark (gold standard)
- Youden's J (TPR - FPR) per category
- Queries pattern_findings + resolved_flow_audit from .pf/repo_index.db

### Known Gaps (TN Deficit)
- cmdi: 9 TP, 0 TN — no safe command execution patterns exist
- ssrf: 8 TP, 0 TN — no proper URL validation patterns exist
- xss: 3 TP, 0 TN — no escaped output patterns exist
- crypto: 4 TP, 0 TN — no SHA-256/bcrypt patterns exist
- weakrand: 3 TP, 0 TN — no CSPRNG patterns exist
- redos: 1 TP, 0 TN — minimal coverage
- infodisclosure: 3 TP, 0 TN — minimal coverage

These gaps mean FPR cannot be measured for those categories. Safe variants needed in Phase 3.

### Source Annotations Added
- vuln-code-snippet start/end markers added to annotated files
- Keys match ground truth YAML exactly

### Phase 1 Shortcomings (acknowledged)
- Copied apps blindly without reading all source code
- 3 apps (calorie_app, jobqueue, anarchy_commerce) never audited
- No documentation of app architectures or data flows
- Ground truth based on partial code reading

---

## 2026-03-19 — Proper Audit & Documentation (Phase 2)

### Full Security Audit Performed
Every .rs file across all 8 apps read and classified. Function-level security audit with CWE assignments. Findings:

**rust_calorie_app** — REAL production app, not intentionally vulnerable
- 3 CRITICAL SQL injection vulns in workout_repo.rs + schedule_repo.rs (format! with weak `replace("'","''")` escaping)
- 1 hardcoded JWT secret in config.rs
- 15+ properly parameterized query functions (natural TN source)
- Architecture: handlers -> validation -> services -> repositories -> SQLite
- bcrypt auth, input validation, proper error handling

**rust_jobqueue** — INTENTIONALLY vulnerable job queue
- 25+ vulnerabilities across 6 CWE categories
- SQLi: queries.rs (all builder methods), sqlite.rs (search, tags, ordered, execute_raw), handlers (search, execute_sql)
- Path traversal: backup.rs (create, restore, read, write, delete, compress — 7 functions)
- Command injection: backup.rs (export_to_sql, import_from_sql — sh -c with format!)
- Weak crypto: auth.rs (MD5 passwords, predictable tokens, JWT alg:none)
- SSRF: worker/handlers.rs (HTTP handler with user URL)
- Info disclosure: error.rs (SQL in responses), middleware.rs (logs query params), config.rs (logs API key)

**anarchy_commerce** — Microservices with real-world patterns
- Hardcoded Stripe secret key in payments service
- Elasticsearch injection in search service (user input in ES JSON query)
- Missing input validation on payment amounts

**deepflow-rust** — EXCLUDED from scoring
- All functions are taint flow tests, not vulnerability patterns
- Many sinks are simulated (println! not real SQL)
- Value is taint pipeline stress testing, not TP/TN benchmark scoring

### Annotations Added (Phase 2)
| App | Files | Test Cases | Notes |
|-----|-------|------------|-------|
| rust_calorie_app | 5 files (workout_repo, schedule_repo, user_repo, meal_repo, config) | 12 (4 TP + 8 TN) | Real app with natural TP/TN mix |
| rust_jobqueue | 7 files (sqlite, backup, queries, auth, middleware, routes, error) | 20 TP | Intentionally vulnerable |
| anarchy_commerce | 2 files (payments, search) | 3 TP | Microservices patterns |

### Ground Truth Updated
- v0.2 -> v0.3: added 35 new test cases
- Total: 98 test cases (64 Phase 1 + 34 Phase 2)
- 98 annotations = 98 ground truth entries (verified, 1:1 match)
- New category: inputval (CWE-20) — 1 test case

### Inventory After Phase 2
| Category | CWE | TP | TN | Total |
|----------|-----|----|----|-------|
| sqli | 89 | 20 | 15 | 35 |
| cmdi | 78 | 11 | 0 | 11 |
| pathtraver | 22 | 11 | 1 | 12 |
| ssrf | 918 | 7 | 0 | 7 |
| memsafety | 119 | 7 | 4 | 11 |
| crypto | 327/347 | 5 | 0 | 5 |
| weakrand | 330 | 3 | 0 | 3 |
| xss | 79 | 2 | 0 | 2 |
| infodisclosure | 200/209/532/798 | 5 | 0 | 5 |
| deser | 502 | 2 | 0 | 2 |
| intoverflow | 190 | 2 | 0 | 2 |
| redos | 1333 | 1 | 0 | 1 |
| inputval | 20 | 1 | 0 | 1 |
| **TOTAL** | | **77** | **20** | **97** |
*Note: 1 extra annotation (cmdiBackendShellExec) brings total annotations to 98; the 98th was already in Phase 1 ground truth.

### Remaining TN Gaps
sqli now has 15 TN (43% TN ratio) — healthy.
All other categories still 0 TN — FPR unmeasurable.
Phase 3 needed: create safe variants for cmdi, pathtraver, ssrf, crypto, weakrand, xss.

---

## 2026-03-19 — Rule Gap Analysis & Project Documentation (Phase 3)

### Rules Audited
Read `rules_purge.md` and `patterns_sop.md` in TheAuditorV2 root. Read all 8 Rust-dedicated rules + 3 polyglot rules that include `.rs`. Read `taint/patterns/rust.py` (190 sources, 232 sinks, 118 sanitizers).

### Coverage Gap Analysis
Mapped all 13 benchmark categories against actual engine rules. Verified each gap by reading the rule source code.

**Covered (5 categories, 76 of 97 test cases):** sqli, cmdi, pathtraver, ssrf, memsafety
**Full gaps (7 categories, 19 test cases):** xss, weakrand, deser, redos, infodisclosure, inputval
**Partial gaps (2 categories, 7 test cases):** crypto (dep-level only), intoverflow (crypto context only)

Gap details documented in rule coverage matrix (see rust_benchmark.md).

### Rule Coverage Matrix
Added to `rust_benchmark.md` — maps each benchmark category to the specific rule file, track type, and coverage status.

### Project-Level Documentation
- Updated `README.md` (project root) — added Rust section with 98 test cases, 13 CWEs, 4 frameworks
- Updated directory structure in README to match reality
- Rule coverage matrix added to rust_benchmark.md (8 gaps prioritized by impact)
- LICENSE and CONTRIBUTING.md already created by other teams

### Rust Pattern Quality (from patterns_sop.md audit)
`taint/patterns/rust.py` is CLEAN:
- DC-1 (bare words): None found — all patterns use qualified names
- DC-2 (wrong language): None — all Rust-specific (`::` qualified, `!` macros)
- DC-4 (misclassified): Minor — panic macros as sinks (could be availability sources)
- DC-5 (contradictions): None
- Overall: cleanest pattern file of all 6 languages

---

## 2026-03-19 — Due Diligence & Roadmap (Phase 4)

### Self-Assessment
Compared Rust benchmark structure against OWASP Java Benchmark gold standard. Identified 8 structural gaps that prevent the benchmark from being taken seriously at industry level.

### Verification Performed
- Spot-checked 30 of 98 annotations across 5 representative files
- ALL 30 verified CORRECT (classifications, line targets, function wrapping)
- Annotation quality is solid — the problems are structural, not data quality

### Structural Analysis (Java vs Rust)
| Dimension | Java Gold | Rust Current |
|-----------|-----------|--------------|
| Test cases | 2,740 | 97 (28x smaller) |
| TP/TN balance | 52/48 | 79/21 |
| Test identity | Filename (immutable) | Annotation (mutable) |
| Answer key | Flat CSV | YAML + annotations |
| CWE coverage | 11/11 complete | 5/13 complete |

### dev_roadmap.md Created
6 milestones defined, each independently valuable:
- M1: Foundation Verification (baseline run) — NEXT
- M2: TN Parity (50+ safe variants) — CRITICAL
- M3: Answer Key Migration (YAML -> CSV)
- M4: Dedicated Test Files (optional, high effort)
- M5: Scale to 300+ test cases
- M6: CI & Automation

### Honest Assessment
We are ~25% done. The 98 annotated test cases are correct and verified. But the benchmark cannot measure false positive rates for 69% of its categories, has no standalone answer key, and is 28x smaller than the gold standard. The roadmap charts the path from here to publishable.

---

## 2026-03-19 — M2: TN Parity (Phase 5)

### Safe Variants Created
21 isomorphic safe functions added across 6 files. Each safe function mirrors its vulnerable counterpart — same API shape, same control flow — with ONE security-relevant difference.

| File | Safe Functions | Categories |
|------|---------------|------------|
| commands.rs | 5 (allowlist, shell escape, path validate, metachar filter, target format) | cmdi |
| network.rs | 5 (domain allowlist, size limit, trusted list, host:port allow, strict parse) | ssrf |
| rocket_test/main.rs | 2 (HTML escape, bcrypt-strength hash) | xss, crypto |
| warp_test/main.rs | 2 (HTML escape, SHA-256) | xss, crypto |
| vulnerable.rs | 3 (pattern limit, typed deser, checked arithmetic) | redos, deser, intoverflow |
| auth.rs | 4 (random bytes token, strong hash, random key, HS256 JWT) | weakrand, crypto |

### Inventory After M2
- Total test cases: **119** (77 TP + 42 TN)
- TP/TN ratio: **65/35** (was 79/21)
- Categories with TN: **11 of 13** (was 4 of 13)
- FPR measurable: **94%** of test cases (was 31%)
- Remaining TN gaps: infodisclosure (5 TP, 0 TN), inputval (1 TP, 0 TN)
- All 119 annotations verified 1:1 with ground truth YAML (zero orphans)

---

## 2026-03-19 — M3: Answer Key Migration (Phase 6)

### CSV Answer Key Created
- `expectedresults-0.1.csv` — 119 test cases in flat CSV format
- Format: `test_name,category,real_vulnerability,CWE` (matches Java/Go benchmarks exactly)
- Sorted by category then key name
- 78 TP + 41 TN (note: sqli has 21 TP due to ES injection counted under sqli)

### Scoring Script Updated
- Replaced YAML parser (~20 lines) with CSV parser (~10 lines)
- Simpler, faster, matches Java benchmark scoring pattern
- Annotation-based line range detection unchanged
- taint_sinks still excluded (M1 fix preserved)

### YAML Downgraded to Documentation
- `rust_ground_truth.yml` header updated: "Scoring uses CSV. This file is documentation only."
- YAML still has descriptions and methodology notes not in CSV
- Both files must stay in sync when test cases are added

### Files Changed
- NEW: `expectedresults-0.1.csv`
- MODIFIED: `rust_benchmark.md` (scoring script)
- MODIFIED: `rust_ground_truth.yml` (header note, version 0.3)
- MODIFIED: `CHANGELOG.md` (this entry)

---

## 2026-03-19 — M4: Dedicated Test Case Files (Phase 7)

### testcode/ Directory Created
Framework-agnostic standalone test files matching Java/Go benchmark pattern.

**Design:** Each test is a pure function: `BenchmarkRequest -> BenchmarkResponse`. No framework dependency. `shared.rs` provides the abstraction types. Thin adapters (not yet written) convert framework HTTP to BenchmarkRequest.

### Files Created
| File | Category | TP/TN | Pattern |
|------|----------|-------|---------|
| `shared.rs` | - | - | BenchmarkRequest, BenchmarkResponse types |
| `sqli_001_vulnerable.rs` | sqli | TP | format!() SQL with user param |
| `sqli_002_safe.rs` | sqli | TN | Parameterized ? placeholder |
| `cmdi_001_vulnerable.rs` | cmdi | TP | Command::new(user_input) |
| `cmdi_002_safe.rs` | cmdi | TN | Command from match allowlist |
| `pathtraver_001_vulnerable.rs` | pathtraver | TP | fs::read_to_string(user_path) |
| `pathtraver_002_safe.rs` | pathtraver | TN | canonicalize + starts_with |
| `xss_001_vulnerable.rs` | xss | TP | Raw HTML interpolation |
| `xss_002_safe.rs` | xss | TN | HTML entity escaping |
| `crypto_001_vulnerable.rs` | crypto | TP | MD5-like weak hash |
| `crypto_002_safe.rs` | crypto | TN | SHA-256/bcrypt-strength hash |

### Scoring Updated
- Scoring script now scans both `apps/` and `testcode/` for annotations
- CSV updated with 10 new entries (5 TP + 5 TN)
- YAML updated with 10 new entries

### Inventory After M4
- Total test cases: **129** (83 TP + 46 TN)
- TP/TN ratio: **64/36** (was 65/35)
- Sources: apps/ (119 test cases) + testcode/ (10 test cases)
- All 129 annotations = 129 CSV entries = 129 YAML entries (triple-verified)

---

## 2026-03-19 — M5: Scale to 262 Test Cases (Phase 8)

### 133 New Standalone Test Files
Written in 5 batches across all 13 categories in `testcode/`:

| Batch | Categories | New Files | Pattern Diversity |
|-------|-----------|-----------|-------------------|
| 1 | infodisclosure, inputval, redos | 28 | Stack traces, env dumps, regex bombs, validation patterns |
| 2 | deser, intoverflow, weakrand | 29 | bincode, YAML bomb, truncating casts, CSPRNG vs timestamp |
| 3 | crypto, xss | 19 | DES/RC4/ECB vs AES-GCM/ChaCha20, attribute/script context XSS |
| 4 | cmdi, pathtraver | 26 | Env injection, piped cmds; symlinks, UUID filenames, chroot |
| 5 | sqli, ssrf, memsafety | 31 | ORDER BY/LIKE injection; DNS rebinding; from_raw_parts |

### Final Inventory
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
| redos | 1333 | 4 | 6 | 10 | 40/60 |
| inputval | 20 | 4 | 6 | 10 | 40/60 |
| **TOTAL** | | **116** | **146** | **262** | **44/56** |

### Key Metrics
- **262 test cases** (was 129) — 2x growth
- **TP/TN ratio: 44/56** (Java gold standard: 52/48) — balanced
- **All 13 categories** have both TP and TN — FPR measurable for 100%
- **Every category >= 10 test cases** — statistically meaningful
- **262 annotations = 262 CSV entries** — zero orphans
- **Sources:** apps/ (119) + testcode/ (143)

---

## 2026-03-19 — M6: CI & Automation (Phase 9)

### Validation Script Created
`scripts/validate_rust.py` — standalone Python 3 script (no dependencies):
- Parses CSV answer key
- Scans all .rs files for vuln-code-snippet annotations
- Checks 1:1 match between CSV keys and annotation keys
- Detects: orphan CSV entries, orphan annotations, unclosed snippets, duplicates
- Prints per-category TP/TN stats
- Exit 0 on pass, 1 on fail

**First run: PASS** — 262 CSV entries, 262 annotations, zero errors.

### GitHub Actions Workflow
`.github/workflows/validate.yml` — runs on push to main and PRs:
- Matrix strategy ready for go/bash when their scripts exist
- Uses Python 3.12, runs validate script, fails PR if validation errors

### PR Template
`.github/PULL_REQUEST_TEMPLATE.md` — checklist for contributors:
- Test file + CSV entry required
- TP/TN pair encouraged
- No vulnerability hints in source
- Local validation run required

### Files Created
- `scripts/validate_rust.py`
- `.github/workflows/validate.yml`
- `.github/PULL_REQUEST_TEMPLATE.md`

---

## 2026-03-19 — Pre-Flight Check (Phase 10)

### Due Diligence Audit
Read every shared file. Verified actual numbers against claims. Found and fixed:

**README.md (shared):**
- Rust section was stale (showed 129 test cases from Phase 2, actual: 262)
- Updated Rust per-category table to verified numbers from validate_rust.py
- Fixed directory structure: added `expectedresults-0.1.csv`, `testcode/`, `dev_roadmap.md`
- Fixed "Combined Scale" table: Rust 129 -> 262, total 732 -> 865
- Go team had already fixed Go numbers, added Project Status + Limitations sections

**validate_go.py (new):**
- Go uses filename-based identity (like Java), not annotations
- First version used annotation scanning — failed (424 orphans)
- Rewrote to match CSV key BenchmarkTestNNNNN -> file benchmark_test_nnnnn.go
- Now passes: 424/424, 16 categories, 50/49 TP/TN

**CI workflow updated:**
- `.github/workflows/validate.yml` now runs both Rust and Go validation
- Bash deferred until validate_bash.py exists

### Validation Results (final)
```
Rust: PASS — 262 CSV, 262 annotations, 116 TP, 146 TN
Go:   PASS — 424 CSV, 424 test files, 213 TP, 211 TN
Bash: No validator yet (uses YAML, different approach needed)
```
