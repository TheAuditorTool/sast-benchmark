# Scorecard: TheAuditor vs Go Benchmark v0.3.2

> **Tool**: TheAuditor
> **Benchmark**: Go SAST Benchmark v0.3.2 (534 test cases, 24 CWE categories)
> **Date**: 2026-04-01
> **Command**: `aud full --offline`
> **Tuning**: None. Out-of-the-box results with no benchmark-specific adjustments.

## Why We Publish This

OWASP publishes every tool's score against BenchmarkJava -- Checkmarx, Fortify, Veracode, SonarQube, all of them. Honest scores build credibility. Hidden scores destroy it.

We built this benchmark and we ran our own tool against it. Here are the results, untuned, with every false negative and false positive visible.

## Scorecard

```
Category         CWE    TP    FP    FN    TN      TPR     FPR   Score
---------------------------------------------------------------------------
authnfailure     287    6     0     0     6      100.0%    0.0% +100.0%
authzfailure     862    7     0     0     6      100.0%    0.0% +100.0%
cmdi             78     30    0     0     30     100.0%    0.0% +100.0%
codeinj          94     6     0     0     6      100.0%    0.0% +100.0%
csrf             352    7     0     0     6      100.0%    0.0% +100.0%
deserial         502    6     0     0     6      100.0%    0.0% +100.0%
fileupload       434    4     0     0     4      100.0%    0.0% +100.0%
hardcodedcreds   798    6     0     0     6      100.0%    0.0% +100.0%
inputval         20     4     0     0     4      100.0%    0.0% +100.0%
ldapi            90     6     0     0     6      100.0%    0.0% +100.0%
loginjection     117    6     0     0     7      100.0%    0.0% +100.0%
nosql            943    6     0     0     7      100.0%    0.0% +100.0%
pathtraver       22     25    0     0     30     100.0%    0.0% +100.0%
race_condition   362    5     0     0     5      100.0%    0.0% +100.0%
redirect         601    8     0     0     8      100.0%    0.0% +100.0%
securecookie     614    8     0     0     8      100.0%    0.0% +100.0%
sqli             89     65    0     0     58     100.0%    0.0% +100.0%
ssrf             918    10    0     0     10     100.0%    0.0% +100.0%
tlsverify        295    5     0     0     5      100.0%    0.0% +100.0%
trustbound       501    6     0     0     6      100.0%    0.0% +100.0%
weakcipher       327    8     0     0     8      100.0%    0.0% +100.0%
weakhash         328    10    0     0     10     100.0%    0.0% +100.0%
weakrand         330    10    0     0     10     100.0%    0.0% +100.0%
xss              79     13    0     0     15     100.0%    0.0% +100.0%
---------------------------------------------------------------------------
OVERALL                 267   0     0     267    100.0%    0.0% +100.0%
```

**Flat Score: +100.0%** (Youden's J = TPR - FPR)

**Category-Averaged Score: +100.0%** (OWASP standard -- each category weighted equally)

24 of 24 categories at perfect +100.0%. Zero false positives. Zero false negatives. Perfect detection across the entire benchmark.

## What the Scores Mean

**Perfect detection (all 24 categories at +100%)**:

All 24 categories achieve perfect scores -- 100% TPR with 0% FPR. Every vulnerable pattern detected, every safe pattern correctly ignored. This includes:

- **Flow-detection categories**: sqli, cmdi, pathtraver, ssrf, xss, loginjection, ldapi, nosql, redirect, trustbound, codeinj -- interprocedural taint analysis with cross-file resolution and sanitizer awareness.
- **Structural categories**: weakhash, weakrand, weakcipher, tlsverify, securecookie, deserial, hardcodedcreds -- API-usage and configuration detection.
- **Control-flow categories**: authnfailure, authzfailure, csrf, race_condition, inputval, fileupload -- framework-aware validation and access control detection.

## Analysis

**267 True Positives**: TheAuditor correctly identifies 100% of vulnerable Go patterns across all 24 CWE categories.

**0 False Negatives**: Zero vulnerable patterns missed.

**0 False Positives**: Zero false alarms across all 267 safe patterns. Perfect precision.

**267 True Negatives**: 100% of safe patterns correctly ignored.

## How to Reproduce

```bash
cd C:\Users\santa\Desktop\open_tests\gorustbash_benchmark\go
aud full --offline

# Convert and score
python ../scripts/convert_theauditor.py .pf/repo_index.db
python ../scripts/score_sarif.py theauditor.sarif expectedresults-0.3.2.csv
```
