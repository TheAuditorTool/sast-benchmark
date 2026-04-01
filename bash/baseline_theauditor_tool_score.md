# Baseline Score: TheAuditor v3.6

**Tool:** TheAuditor v3.6
**Benchmark:** Bash SAST Benchmark v0.3.1 (356 test cases, 16 categories)
**Date:** 2026-04-01
**Previous:** v3.6 baseline (2026-03-24) scored +50.5%. Prior to that: 100%. This run: +96.0%.
**Configuration:** `aud full --offline` (default rules, no tuning)

---

## Why We Publish This

We built this benchmark. We also built the tool being scored. Publishing our own tool's results — including every missed vulnerability and every false alarm — is how we hold ourselves accountable.

OWASP publishes scores for Checkmarx, Fortify, Veracode, and dozens of other commercial tools against the Java benchmark. Nobody loses credibility for honest numbers. They lose credibility for hiding them.

This baseline is a public roadmap. Every FN below is a detection gap we intend to close. Every FP is a discrimination problem we intend to fix. Future versions of TheAuditor will be re-scored against the same benchmark, and the delta will show whether we actually improved or just claimed to.

---

## Scorecard

```
Category             CWE    TP    FP    FN    TN      TPR     FPR   Score
------------------------------------------------------------------------------
auth_bypass          306    4     0     0     4    100.0%    0.0% +100.0%
cmdi                 78     53    0     0     53   100.0%    0.0% +100.0%
codeinj              94     18    0     0     18   100.0%    0.0% +100.0%
hardcoded_creds      798    7     2     0     5    100.0%   28.6%  +71.4%
infodisclosure       200    6     0     0     9    100.0%    0.0% +100.0%
insecure_perms       732    5     0     0     7    100.0%    0.0% +100.0%
insecure_temp        377    4     0     0     4    100.0%    0.0% +100.0%
pathtraver           22     9     0     0     9    100.0%    0.0% +100.0%
race_condition       362    5     0     0     5    100.0%    0.0% +100.0%
rce                  94     5     0     0     5    100.0%    0.0% +100.0%
sqli                 89     21    0     0     21   100.0%    0.0% +100.0%
ssl_bypass           295    6     0     0     7    100.0%    0.0% +100.0%
ssrf                 918    11    0     0     11   100.0%    0.0% +100.0%
unquoted             78     5     0     5     10    50.0%    0.0%  +50.0%
weakcrypto           327    6     0     0     6    100.0%    0.0% +100.0%
weakrand             330    5     0     0     5    100.0%    0.0% +100.0%
------------------------------------------------------------------------------
OVERALL                     170   2     5     179   97.1%    1.1%  +96.0%
```

**Score: +96.0%** (Youden's J = TPR - FPR)

---

## Regressions from 100% Baseline

Two categories regressed from the prior 100% score:

### unquoted (+50.0%) — 5 FN

| Test Case | File | Lines |
|-----------|------|-------|
| docker_unquoted_vars | apps/pipeline-manager/lib/deploy.sh | 294-339 |
| unquoted_tar_output | apps/pipeline-manager/scripts/backup.sh | 104-117 |
| restore_db_unquoted_cp | apps/pipeline-manager/scripts/backup.sh | 190-206 |
| restore_full_mkdir_unquoted | apps/pipeline-manager/scripts/backup.sh | 218-231 |
| unquoted_rm_variable | testcode/unquoted_tests.sh | 41-46 |

### hardcoded_creds (+71.4%) — 2 FP

| Test Case | File | Lines |
|-----------|------|-------|
| hardcoded_password_from_env | testcode/hardcoded_creds_tests.sh | 14-21 |
| hardcoded_creds_env_required | testcode/hardcoded_creds_tests.sh | 101-109 |

---

## What The Numbers Mean

- **170 True Positives**: Vulnerabilities correctly detected
- **179 True Negatives**: Safe code correctly ignored
- **5 False Negatives**: Vulnerabilities missed — all in unquoted category
- **2 False Positives**: Safe code incorrectly flagged — both in hardcoded_creds

---

## Reproduction

```bash
# 1. Install TheAuditor v3.6
# 2. Setup and scan
cd gorustbash_benchmark/bash
aud setup-ai --target . --sync
aud full --offline

# 3. Score
python3 bash_benchmark.py
```
