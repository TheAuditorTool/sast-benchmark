# Bash SAST Benchmark — Development Roadmap

> **Created**: 2026-03-19
> **Team**: Bash
> **Status**: Phase 0 complete (foundation). Phases 1-5 pending.
> **Goal**: Public OWASP-style benchmark for bash shell script SAST. World-first.

---

## Current State (Honest Assessment)

**What exists**: 158 test cases (112 TP / 46 TN) across 13 CWE categories, annotated source files, scoring script, engine analysis documentation, 17 verified coverage gaps.

**What's wrong**:
1. **1 classification error**: `cmdi_arithmetic_expansion` incorrectly marked vulnerable
2. **3 annotation placement issues**: markers on wrong lines in 3 test cases
3. **~25 missing attack patterns** that any serious bash SAST benchmark needs
4. **TP/TN imbalance**: 70.9%/29.1% (target: ~55/45)
5. **Zero baseline scores** — haven't run `aud full` against the benchmark yet
6. **No FN/FP root cause analysis** — can't do until baseline exists

**What's good**:
- Original file annotations: 100% verified correct (58/58 across 9 files)
- Ground truth YAML: 158/158 keys match source annotations
- Scoring script: Loads correctly, RULE_MAP verified against real DB
- Engine analysis: 27 rules documented, 17 gaps with CWE/pipeline-stage/fix-complexity
- Coverage gaps shared in root `coverage_cve_gaps.md` with Go/Rust teams

**Completion estimate**: ~25% done. Phases below bring it to production quality.

---

## Phase 1: Fix Known Errors

**Scope**: 4 targeted fixes. No new test cases.
**Effort**: 1 session, ~30 minutes.

### 1.1 Fix `cmdi_arithmetic_expansion` Classification

**Problem**: Bash arithmetic `$(( user_expr ))` does NOT execute commands from string variables. The variable `user_expr` is evaluated as an arithmetic expression, not a command. This is not CWE-78.

**However**: Bash arithmetic CAN be dangerous in specific ways:
- `$(( a[$(cmd)] ))` — array subscript can execute commands
- Variables referenced in arithmetic are recursively expanded if they contain arithmetic expressions
- Example: `user_var="a]"; a[0]=$(malicious)"; $(( arr[$user_var] ))` can execute code

**Decision**: Rewrite the test code to show the ACTUAL exploitable pattern (recursive arithmetic variable expansion), not the non-exploitable pattern currently written. Keep vulnerable=true but fix the code.

**Files**: `adversarial/cmdi_tests.sh`, `bash_ground_truth.yml`

### 1.2 Fix `codeinj_heredoc_expansion` Annotation

**Problem**: vuln-line marker is on a comment line (line 76) instead of on the actual `$(${user_cmd})` line inside the heredoc.

**Fix**: Move marker to the heredoc body line containing the expansion.

**File**: `adversarial/codeinj_tests.sh`

### 1.3 Fix `sqli_table_name_injection` Annotation

**Problem**: No inline `# vuln-code-snippet vuln-line` marker on the sqlite3 line.

**Fix**: Add inline marker.

**File**: `adversarial/sqli_tests.sh`

### 1.4 Verify `cmdi_backtick_injection` Annotation

**Problem**: Marker is after backtick assignment. Borderline acceptable since backtick syntax makes inline comments tricky.

**Fix**: Verify placement is the best possible given backtick syntax constraints. Document if can't improve.

**File**: `adversarial/cmdi_tests.sh`

---

### Phase 1 Completion Notes (2026-03-19)

**Re-verification found 3 of 4 issues were false alarms from the audit agent:**

1. `cmdi_arithmetic_expansion` — CORRECTLY classified as vulnerable. Bash arithmetic recursively evaluates variable names; array subscripts with `$(cmd)` execute commands. Added explanatory comment to ground truth.
2. `codeinj_heredoc_expansion` — Moved vuln-line marker from comment line to `cat << EOF` line (the security decision point). Heredoc bodies can't contain inline comments.
3. `sqli_table_name_injection` — Inline marker already existed. Audit agent missed it.
4. `cmdi_backtick_injection` — Inline marker already existed. No change needed.

**Lesson**: Audit agents can be wrong. Always verify against actual source code (Prime Directive).

---

## Phase 2: Tier 1 Coverage Expansion

**Scope**: Add 8 high-priority missing attack patterns + safe variants = ~16 new test cases.
**Effort**: 1 session, ~1 hour. Plan mode first.

Each new test case follows the pattern: vulnerable function + safe variant, both annotated.

### 2.1 `find -exec` with Tainted Arguments (CWE-78)
```
# Vulnerable: find . -name "*.log" -exec ${user_cmd} {} \;
# Safe: find . -name "*.log" -exec cat {} \;  (hardcoded command)
```

### 2.2 Indirect Eval (CWE-94)
```
# Vulnerable: cmd_ref="$user_input"; eval "$cmd_ref"
# Safe: eval "echo hello"  (hardcoded string)
```

### 2.3 Parameterized Query Safe Pattern (CWE-89 TN)
```
# Safe: sqlite3 "$db" "SELECT * FROM users WHERE id = ?" "$user_id"
#   (Note: sqlite3 CLI doesn't actually support ? params, but this tests the concept)
# Better safe: printf %q escaping before SQL interpolation
```

### 2.4 eval with Command Substitution (CWE-94)
```
# Vulnerable: eval "$(curl -s $url)"
# Safe: output=$(curl -s $url); echo "$output"  (display only)
```

### 2.5 grep Regex Injection (CWE-78)
```
# Vulnerable: grep -P "$user_pattern" /var/log/app.log
# Safe: grep -F "$user_string" /var/log/app.log  (-F for literal string)
```

### 2.6 TOCTOU Race on Temp File (CWE-367)
```
# Vulnerable: [ -f /tmp/lockfile ] || touch /tmp/lockfile  (race between check and create)
# Safe: mktemp-based approach with atomic creation
```

### 2.7 Credentials in Heredoc (CWE-798)
```
# Vulnerable: cat << EOF > /tmp/config
#   password = SuperSecret123
#   EOF
# Safe: cat << EOF > /tmp/config
#   password = ${DB_PASSWORD}
#   EOF  (from env, not hardcoded)
```

### 2.8 source with Process Substitution (CWE-94)
```
# Vulnerable: source <(curl -s "$url")
# Safe: source /etc/app/static-config.sh  (hardcoded path)
```

**Files to modify**: New additions to existing adversarial files (add to cmdi_tests.sh, codeinj_tests.sh, sqli_tests.sh, etc.) + `bash_ground_truth.yml` updates.

---

## Phase 3: Tier 2 Coverage Strengthening

**Scope**: Add 8 more patterns + safe variants = ~16 new test cases.
**Effort**: 1 session, ~1 hour. Plan mode first.

### 3.1 `${var@Q}` Sanitizer (bash 4.4+) — Safe pattern for cmdi
### 3.2 `find -name "$pattern"` — Glob injection via find
### 3.3 `CURL_CA_BUNDLE=""` — SSL environment bypass
### 3.4 Credentials in sourced .env files
### 3.5 `printf '%s'` — Safe formatting TN
### 3.6 `command -p` — PATH bypass
### 3.7 tar member name traversal (CWE-22)
### 3.8 ORDER BY SQL injection

**Files**: Same pattern as Phase 2 — add to existing adversarial files.

---

## Phase 4: Polish + Documentation

**Scope**: Standardize all annotations, update ground truth, recount stats, update BENCHMARK.md.
**Effort**: 1 session, ~45 minutes.

### 4.1 Annotation Standardization
- Verify every vuln-line/safe-line is on the actual dangerous/safe line (no comment-line markers)
- Verify every annotation block properly wraps the function
- Verify no overlapping ranges that could confuse scoring

### 4.2 Ground Truth Recount
- Total test cases (target: ~190-200)
- TP/TN split (target: ~55/45)
- Expected FN count
- Expected FP count

### 4.3 Scoring Script Verification
- Run `bash_benchmark.py` — confirm all test cases load
- Confirm annotation count matches ground truth count
- Confirm RULE_MAP is complete

### 4.4 BENCHMARK.md Final Update
- Update all stats tables
- Update detection coverage matrix with new test cases
- Update changelog

### 4.5 coverage_cve_gaps.md Update
- Update gap counts
- Mark any gaps that new test cases now cover

---

## Phase 5: Baseline Run + Analysis

**Scope**: Run `aud full --offline`, establish baseline scorecard, analyze every FN/FP.
**Effort**: 1 session, ~1 hour (depends on aud runtime).

### 5.1 Run aud full --offline
Tell human to run on the benchmark directory.

### 5.2 Run Scoring Script
Execute `bash_benchmark.py` against the new DB.

### 5.3 Scorecard Documentation
Record baseline in BENCHMARK.md with full category breakdown.

### 5.4 FN Root Cause Analysis
For EVERY false negative:
- Which test case key?
- What CWE?
- Why wasn't it detected? (Rule gap? Taint gap? AST extractor gap?)
- Which pipeline stage (from correctness_sop.md)?
- Is this already documented in coverage_cve_gaps.md?

### 5.5 FP Root Cause Analysis
For EVERY false positive:
- Which test case key?
- What was incorrectly flagged?
- Why was it flagged? (Rule too broad? Missing sanitizer recognition?)
- Should the ground truth be changed, or should the rule be fixed?

### 5.6 Delta From Prediction
Compare actual scores against the predicted baseline (30-50% overall).
Document surprises — categories that scored better or worse than expected.

---

## Phase Tracking

| Phase | Description | Status | Est. New Test Cases | Session |
|-------|------------|--------|---------------------|---------|
| 0 | Foundation | DONE | 158 | Iterations 1-3 |
| 1 | Fix Known Errors | DONE | 0 (fixes only) | See notes below |
| 2 | Tier 1 Coverage | DONE | 14 | See notes below |
| 3 | Tier 2 Coverage | DONE | 7 | See notes below |
| 4 | Polish + Docs | DONE | 0 (docs only) | All stats updated to 179 |
| 5 | Baseline Run | PENDING | 0 (scoring only) | After Phase 4 |

**Actual**: 179 test cases (122 TP / 57 TN = 68.2% / 31.8%). 14 expected FN, 3 expected FP.
**Original target was ~190-200** but Prime Directive verification rejected ~11 proposed patterns that couldn't be confirmed exploitable with certainty. Quality over quantity.

---

## What Success Looks Like

When this benchmark is published, a reviewer should be able to:

1. **Read BENCHMARK.md** and understand the entire benchmark: what it tests, how scoring works, what the engine detects, what it misses, and why
2. **Read bash_ground_truth.yml** and for every test case understand: what's being tested, whether it's vulnerable or safe, which CWE applies, and whether the engine is expected to catch it
3. **Run the scoring script** and get a reproducible scorecard
4. **See every FN** documented with root cause (pipeline stage) and fix complexity
5. **See every FP** documented with why the engine over-flagged
6. **Trust the classifications** — every vulnerable=true is genuinely exploitable, every vulnerable=false is genuinely safe
7. **Use it to improve their own tool** — the benchmark works for ANY bash SAST tool, not just TheAuditor

This is the standard. We're not there yet, but the roadmap gets us there.
