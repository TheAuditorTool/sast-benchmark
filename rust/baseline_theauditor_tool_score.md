# Rust Benchmark Scorecard: TheAuditor

**Tool:** TheAuditor v3.5
**Benchmark:** 262 test cases, 13 CWE categories, 4 frameworks
**Scoring:** SARIF pipeline (`convert_theauditor.py` + `score_sarif.py`)

---

## Current Score (2026-03-23)

```
Category         CWE    TP    FP    FN    TN      TPR     FPR   Score
----------------------------------------------------------------------
cmdi             78     14    16    0     0    100.0%  100.0%   +0.0%
crypto           327    9     9     0     2    100.0%   81.8%  +18.2%
deser            502    4     3     2     3     66.7%   50.0%  +16.7%
infodisclosure   200    8     0     0     8    100.0%    0.0% +100.0%
inputval         20     4     0     0     6    100.0%    0.0% +100.0%
intoverflow      190    5     2     0     5    100.0%   28.6%  +71.4%
memsafety        119    8     6     0     6    100.0%   50.0%  +50.0%
pathtraver       22     14    14    0     0    100.0%  100.0%   +0.0%
redos            1333   5     1     0     4    100.0%   20.0%  +80.0%
sqli             89     22    19    2     7     91.7%   73.1%  +18.6%
ssrf             918    6     6     3     7     66.7%   46.2%  +20.5%
weakrand         330    7     4     0     5    100.0%   44.4%  +55.6%
xss              79     5     9     0     2    100.0%   81.8%  +18.2%
----------------------------------------------------------------------
TOTAL                   111   89    7     55    94.1%   61.8%  +32.3%

MACRO AVERAGE (OWASP):  94.2%  52.0%  +42.2%
```

Score = TPR - FPR (Youden's J). 0% = random guessing. +100% = perfect.

---

## Baseline Score (2026-03-20, v0.3 ground truth)

```
Category         CWE    TP    FP    FN    TN      TPR     FPR   Score
----------------------------------------------------------------------
cmdi             78     8     6     6     10    57.1%   37.5%  +19.6%
crypto           327    0     0     9     11     0.0%    0.0%   +0.0%
deser            502    0     0     5     7      0.0%    0.0%   +0.0%
infodisclosure   200    0     0     8     8      0.0%    0.0%   +0.0%
inputval         20     0     0     4     6      0.0%    0.0%   +0.0%
intoverflow      190    0     0     5     7      0.0%    0.0%   +0.0%
memsafety        119    8     5     0     7    100.0%   41.7%  +58.3%
pathtraver       22     0     0     14    14     0.0%    0.0%   +0.0%
redos            1333   0     0     5     5      0.0%    0.0%   +0.0%
sqli             89     0     0     24    26     0.0%    0.0%   +0.0%
ssrf             918    0     0     9     13     0.0%    0.0%   +0.0%
weakrand         330    0     0     7     9      0.0%    0.0%   +0.0%
xss              79     0     0     5     11     0.0%    0.0%   +0.0%
----------------------------------------------------------------------
OVERALL                 16    11   101   134    13.7%    7.6%   +6.1%
```

---

## Version History

| Date | Score Method | Macro Avg | Flat Score | TP | FP | FN | TN | Notes |
|------|-------------|-----------|------------|----|----|----|----|-------|
| 2026-03-20 | Legacy DB | N/A | +6.1% | 16 | 11 | 101 | 134 | Baseline, untuned, v0.3 ground truth |
| 2026-03-23 | SARIF | +41.7% | +32.3% | 111 | 89 | 7 | 55 | v0.3.2 ground truth, SARIF pipeline |
