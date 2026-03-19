# SAST Coverage & CWE Gap Analysis

Cross-team verified gap analysis: benchmark test cases vs SAST tool rule coverage.

Every gap listed here was verified by reading the SAST tool's rule source code and confirming no rule produces findings for the test case category. Gaps are opportunities — each one, when fixed, makes the tool stronger.

---

## How to Read This Document

| Column | Meaning |
|--------|---------|
| Category | Benchmark scoring category |
| CWE | Common Weakness Enumeration ID |
| Test Cases | TP (true positive) + TN (true negative) count in benchmark |
| Rule | SAST rule that covers this category (or NONE) |
| Gap Type | NONE (covered), PARTIAL (some coverage), FULL (no coverage) |
| Fix Complexity | LOW (add .rs to target_extensions), MEDIUM (new check in existing rule), HIGH (new rule needed) |

---

## Rust (Team Rust)

**Engine rules audited:** 8 dedicated Rust rules + 3 polyglot rules covering `.rs`
**Rules audited:** 8 dedicated Rust rules + 3 polyglot rules covering `.rs` files
**Taint patterns:** 190 source patterns, 232 sink patterns, 118 sanitizer patterns

### Covered Categories (no gaps)

| Category | CWE | Test Cases | Rule | Track | Evidence |
|----------|-----|------------|------|-------|----------|
| sqli | 89 | 35 (20+15) | `rust/rust_injection_analyze.py` | taint+structural | Track A: `get_confirmed_flows("SQL Injection")`. Track B: `format!` + `sqlx::query` detection |
| cmdi | 78 | 11 (11+0) | `rust/rust_injection_analyze.py` | taint+structural | Track A: `get_confirmed_flows("Command Injection")`. Track B: `Command::new` structural |
| pathtraver | 22 | 12 (11+1) | `security/path_traversal_analyze.py` | taint | Polyglot Track A, includes `.rs` in target_extensions |
| ssrf | 918 | 7 (7+0) | `security/ssrf_analyze.py` | taint | Polyglot Track A, includes `.rs` in target_extensions |
| memsafety | 119 | 11 (7+4) | `rust/memory_safety.py` + `rust/unsafe_analysis.py` + `rust/ffi_boundary.py` | structural | 3 rules: transmute, raw pointers, unsafe blocks, FFI boundaries |

### Full Gaps (no rule coverage)

#### GAP-R1: XSS (CWE-79) — 2 test cases, 0 coverage
**Benchmark tests:** `xssRocketGreetVulnerable` (Rocket RawHtml), `xssWarpEchoHeader` (Warp reply::html)
**Patterns exist:** 5 XSS sink patterns in `rust.py` (askama, tera, handlebars, `Html(`)
**Missing:** No rule queries `get_confirmed_flows("Cross-Site Scripting (XSS)")` for `.rs` files. The polyglot `xss_analyze.py` does NOT include `.rs` in target_extensions.
**Fix:** MEDIUM — Either add `.rs` to `xss_analyze.py` target_extensions, or add XSS Track A to `rust_injection_analyze.py`

#### GAP-R2: Weak Random (CWE-330) — 3 test cases, 0 coverage
**Benchmark tests:** `weakrandJobqueueToken` (timestamp-based auth token), `weakrandJobqueueApiKey` (timestamp+name API key), `weakrandJobqueueRequestId` (predictable request ID)
**Missing:** No rule detects predictable token/key generation. No structural check for `rand::thread_rng` vs `OsRng` in security contexts. Supply chain rule detects weak crypto CRATES but not weak RNG CODE PATTERNS.
**Fix:** HIGH — New rule needed. Could be structural: detect `SystemTime::now()` or `thread_rng()` used in token/key generation contexts

#### GAP-R3: Insecure Deserialization (CWE-502) — 2 test cases, 0 coverage
**Benchmark tests:** `deserBackendUnsafeDeserialize` (serde_json::from_slice on raw bytes), `deserBackendDangerousDeserialize` (bincode::deserialize)
**Patterns exist:** 20 deserialization sink patterns in `rust.py` (serde_json, serde_yaml, bincode, postcard, etc.)
**Missing:** No rule consumes these patterns. No rule queries `get_confirmed_flows("Insecure Deserialization")` for Rust.
**Fix:** MEDIUM — Add deserialization Track A to `rust_injection_analyze.py`, or create dedicated `rust/deserialization_analyze.py`

#### GAP-R4: ReDoS (CWE-1333) — 1 test case, 0 coverage
**Benchmark tests:** `redosBackendRegexDos` (Regex::new with user-controlled pattern)
**Patterns exist:** 4 regex sink patterns in `rust.py` (Regex::new, RegexBuilder, fancy_regex)
**Missing:** `security/redos_analyze.py` exists as a polyglot rule but `.rs` is NOT in its target_extensions.
**Fix:** LOW — Add `.rs` to `redos_analyze.py` target_extensions. Verify it queries the right tables for Rust.

#### GAP-R5: Information Disclosure (CWE-200/209/532/798) — 5 test cases, 0 coverage
**Benchmark tests:** `infodisclosureBackendGetEnv` (exposes env vars), `infodisclosureCalorieJwtSecret` (hardcoded JWT secret), `infodisclosureJobqueueErrorSql` (SQL in error response), `infodisclosureJobqueueLogParams` (logs query params), `infodisclosurePaymentsHardcodedKey` (hardcoded Stripe key)
**Missing:** No rule detects:
- Hardcoded secrets in `.rs` source code (the `secrets/hardcoded_secret_analyze.py` may not cover `.rs`)
- SQL queries leaking in HTTP error responses
- Sensitive data in log output
- Environment variable exposure endpoints
**Fix:** HIGH — Multiple sub-rules needed. Hardcoded secrets rule needs `.rs` support. New structural checks for error message content and log content.

#### GAP-R6: Input Validation (CWE-20) — 1 test case, 0 coverage
**Benchmark tests:** `inputvalPaymentsMissingValidation` (no validation on payment amount)
**Missing:** No rule checks for missing input validation on financial/critical values.
**Fix:** HIGH — New rule needed. Could be structural: detect handler functions that use numeric inputs without range validation.

### Partial Gaps (some coverage, incomplete)

#### GAP-R7: Weak Cryptography (CWE-327/347) — 5 test cases, partial coverage
**Benchmark tests:** `cryptoRocketMd5Login` (MD5 password hash), `cryptoWarpSha1Hash` (SHA-1), `cryptoBackendWeakCrypto` (MD5+XOR+hardcoded key), `cryptoJobqueueMd5Password` (MD5 in auth), `cryptoJobqueueJwtNone` (JWT alg:none)
**Covered:** `supply_chain.py` detects weak crypto CRATES in Cargo.toml (md5, sha1 dependencies)
**Not covered:** Code-level usage of `md5::compute()`, `Sha1::new()`, XOR "encryption", JWT with `alg: "none"`. No rule detects weak algorithm usage in function calls.
**Fix:** MEDIUM — Add structural detection to existing supply_chain rule or create `rust/crypto_analyze.py` that checks `callee_function WHERE_IN` for weak crypto calls

#### GAP-R8: Integer Overflow (CWE-190) — 2 test cases, partial coverage
**Benchmark tests:** `intoverflowBackendOverflow` (user-controlled arithmetic), `intoverflowBackendUncheckedArithmetic` (wrapping ops)
**Covered:** `integer_safety.py` detects truncating casts in crypto/financial contexts
**Not covered:** General arithmetic overflow with user-controlled operands. `wrapping_add`/`wrapping_mul` usage is detected but only as "used" (informational), not as "vulnerable" when operands are user-controlled.
**Fix:** MEDIUM — Extend integer_safety.py to flag wrapping arithmetic in handler contexts (not just crypto)

### Gap Priority (by impact on benchmark score)

| Priority | Gap | Test Cases Affected | Fix Complexity |
|----------|-----|--------------------:|----------------|
| 1 | GAP-R5: Information Disclosure | 5 | HIGH |
| 2 | GAP-R7: Weak Cryptography | 5 | MEDIUM |
| 3 | GAP-R2: Weak Random | 3 | HIGH |
| 4 | GAP-R3: Deserialization | 2 | MEDIUM |
| 5 | GAP-R1: XSS | 2 | MEDIUM |
| 6 | GAP-R8: Integer Overflow | 2 | MEDIUM |
| 7 | GAP-R4: ReDoS | 1 | LOW |
| 8 | GAP-R6: Input Validation | 1 | HIGH |

**Total test cases in gap categories:** 21 of 97 (22%) — these will show as FN in baseline scoring.

---

## Go (Team Go)

*Section to be completed by Go team.*

---

## Bash (Team Bash)

*Section to be completed by Bash team.*

---

## Cross-Language Observations

*To be filled after all teams complete their sections. Look for:*
- *Gaps that appear in ALL languages (systemic engine issues)*
- *Gaps that appear in only ONE language (missing language support)*
- *Patterns where taint pipeline works but rules don't consume the data*
- *Categories where patterns exist but no discoverer classifies them*
