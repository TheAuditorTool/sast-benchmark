# Bash SAST Benchmark — Development Roadmap

> **Created**: 2026-03-19
> **Team**: Bash
> **Status**: v0.5.3 shipped. All phases complete.
> **Goal**: Public OWASP-style benchmark for bash shell script SAST. World-first.

---

## Current State (v0.5.3)

- **867 test cases** (424 TP / 443 TN) across 20 CWE categories
- **TP/TN balance:** 48.9/51.1
- **1-file-1-test architecture** — 867 individual `benchmark_test_NNNNN.sh` files
- **CSV ground truth** (`expectedresults-0.5.3.csv`) is the sole scoring authority
- **Baseline score** established (see `baseline_theauditor_tool_score.md`)
- **Apps moved to `vulnerable_apps/bash/`** (5 apps, 191 entries, scored separately)

The phases below document the development history from initial 158 test cases to current 867.

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

**Files**: `adversarial/cmdi_tests.sh`, `expectedresults-0.3.1.csv`

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

**Lesson**: Automated analysis can be wrong. Always verify classifications against actual bash behavior.

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

**Files to modify**: New additions to existing adversarial files (add to cmdi_tests.sh, codeinj_tests.sh, sqli_tests.sh, etc.) + `expectedresults-0.3.1.csv` updates.

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
- Run `python3 ../scripts/score_sarif.py` — confirm all test cases load
- Confirm annotation count matches ground truth count
- Confirm RULE_MAP is complete

### 4.4 BENCHMARK.md Final Update
- Update all stats tables
- Update detection coverage matrix with new test cases
- Update changelog

### 4.5 Coverage Gap Review
- Review gap counts against updated benchmark categories
- Mark any gaps that new test cases now cover

---

## Phase 5: Baseline Run + Analysis

**Scope**: Run `SAST tool scan`, establish baseline scorecard, analyze every FN/FP.
**Effort**: 1 session, ~1 hour (depends on aud runtime).

### 5.1 Run SAST tool scan
Tell human to run on the benchmark directory.

### 5.2 Run Scoring Script
Run `python3 ../scripts/convert_theauditor.py .pf/repo_index.db` then `python3 ../scripts/score_sarif.py theauditor.sarif expectedresults-0.5.3.csv`.

### 5.3 Scorecard Documentation
Record baseline in BENCHMARK.md with full category breakdown.

### 5.4 FN Root Cause Analysis
For EVERY false negative:
- Which test case key?
- What CWE?
- Why wasn't it detected? (Rule gap? Taint gap? AST extractor gap?)
- Which pipeline stage?
- Is this already documented as a known gap?

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
| 5 | Baseline Run | PENDING | 0 (scoring only) | Ready — all apps annotated |
| 6A | deepflow-webhook app | DONE | 28 | 8 files, 1,347 lines — 17 TP, 11 TN |
| 6B | deepflow-ops app | DONE | 20 | 7 files, 785 lines — 15 TP, 5 TN |
| 6C | dataforge app | DONE | 10 | 4 files, 436 lines — 5 TP, 5 TN |
| 6D | Phase 6 polish | DONE | 0 | All docs updated to 237 |
| 7 | Pre-flight check | DONE | 0 | Hallucinated numbers fixed, CHANGELOG.md created, validate_bash.py created, CI updated, v0.3 |
| 8A | OWASP rebalancing — securepipeline app | DONE | 55 (TN only) | 7 files, CI/CD pipeline, all TN cases |
| 8B | OWASP rebalancing — testcode extensions | DONE | 36 (TN only) | +14 cmdi, +8 codeinj, +4 ssrf, +3 hardcoded_creds, +3 unquoted, +2 sqli, +1 pathtraver, +1 weakcrypto |
| 9 | New CWE: weakrand (CWE-330) | DONE | 10 (5+5) | $RANDOM vs /dev/urandom patterns |
| 10 | New CWE: race_condition (CWE-362) | DONE | 10 (5+5) | TOCTOU, flock, noclobber patterns |
| 11 | New CWE: auth_bypass (CWE-287/306) | DONE | 8 (4+4) | env bypass, empty cred, missing webhook sig |
| 12 | Scoring script updates | DONE | 0 | RULE_MAP/SINK_MAP for 3 new categories |
| 13 | Final documentation + v0.3.1 release | DONE | 0 | All stats finalized |
| 14 | New CWE: loginjection (CWE-117) | DONE | 20 (10+10) | echo/printf/logger/tee to log files |
| 15 | New CWE: privilege_escalation (CWE-250) | DONE | 20 (10+10) | sudo/docker --privileged/su/nsenter/pkexec |
| 16 | New CWE: dos (CWE-770) | DONE | 20 (10+10) | unbounded loops, fork bombs, tar bombs |
| 17 | New CWE: cleartext_tx (CWE-319) | DONE | 20 (10+10) | http/ftp/telnet/nc vs https/sftp/ssh |
| 18 | 10/10 floor expansion (11 categories) | DONE | 90 | insecure_temp, auth_bypass, rce, weakrand, race_condition, insecure_perms, weakcrypto, ssl_bypass, hardcoded_creds, infodisclosure, pathtraver |

| 19 | 25/25 floor expansion (19 categories) | DONE | 530 (265V/265S) | 19 new _extended_tests.sh files |
| 20 | 1-file-1-test restructure (anti-leakage) | DONE | 0 new | 39 multi-test files → 1,058 individual benchmark_test_NNNNN.sh; all comments stripped; annotation system retired; file-based scoring |

**Current state (v0.5.1)**: 1,058 test cases (529 TP / 529 TN = 50.0% / 50.0%). 1,058 individual files, 5 reference apps, 20 CWE categories. Zero target leakage.
**Statistical rationale for 25/25 floor**: At 10/10, one misclassified test = 10% category swing (too noisy). At 25/25, one test = 4% swing (matches OWASP Java methodology minimum for statistically meaningful per-category scores).
**CWE pre-flight corrections**: CWE-269 rejected (Class-level, mapping DISCOURAGED) in favor of CWE-250. CWE-400 rejected (Class-level, mapping DISCOURAGED) in favor of CWE-770.

---

## What Success Looks Like

When this benchmark is published, a reviewer should be able to:

1. **Read BENCHMARK.md** and understand the entire benchmark: what it tests, how scoring works, what the engine detects, what it misses, and why
2. **Read expectedresults-0.3.1.csv** and for every test case understand: what's being tested, whether it's vulnerable or safe, which CWE applies, and whether the engine is expected to catch it
3. **Run the scoring script** and get a reproducible scorecard
4. **See every FN** documented with root cause (pipeline stage) and fix complexity
5. **See every FP** documented with why the engine over-flagged
6. **Trust the classifications** — every vulnerable=true is genuinely exploitable, every vulnerable=false is genuinely safe
7. **Use it to improve their own tool** — the benchmark works for ANY bash SAST tool, not just one tool

This is the standard. We're not there yet, but the roadmap gets us there.
