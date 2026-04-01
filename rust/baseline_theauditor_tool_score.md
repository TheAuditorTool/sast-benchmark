# Rust Benchmark Scorecard: TheAuditor

**Tool:** TheAuditor v3.6
**Benchmark:** 268 test cases, 13 CWE categories, 4 frameworks
**Scoring:** SARIF pipeline (`convert_theauditor.py` + `score_sarif.py`)

---

## Current Score (2026-04-01, round 9) -- PERFECT SCORE

```
Category         CWE    TP    FP    FN    TN      TPR     FPR   Score
----------------------------------------------------------------------
cmdi             78     14    0     0     16   100.0%    0.0% +100.0%
crypto           327    9     0     0     11   100.0%    0.0% +100.0%
deser            502    6     0     0     6    100.0%    0.0% +100.0%
infodisclosure   200    8     0     0     8    100.0%    0.0% +100.0%
inputval         20     4     0     0     6    100.0%    0.0% +100.0%
intoverflow      190    5     0     0     7    100.0%    0.0% +100.0%
memsafety        119    8     0     0     12   100.0%    0.0% +100.0%
pathtraver       22     14    0     0     14   100.0%    0.0% +100.0%
redos            1333   5     0     0     5    100.0%    0.0% +100.0%
sqli             89     23    0     0     27   100.0%    0.0% +100.0%
ssrf             918    9     0     0     13   100.0%    0.0% +100.0%
weakrand         330    7     0     0     9    100.0%    0.0% +100.0%
xss              79     11    0     0     11   100.0%    0.0% +100.0%
----------------------------------------------------------------------
TOTAL                   123   0     0     145  100.0%    0.0% +100.0%

MACRO AVERAGE (OWASP): 100.0%   0.0% +100.0%
```

Score = TPR - FPR (Youden's J). 0% = random guessing. +100% = perfect.

---

## Previous Score (2026-03-25, round 8)

```
Category         CWE    TP    FP    FN    TN      TPR     FPR   Score
----------------------------------------------------------------------
cmdi             78     14    0     0     16   100.0%    0.0% +100.0%
crypto           327    9     0     0     11   100.0%    0.0% +100.0%
deser            502    6     0     0     6    100.0%    0.0% +100.0%
infodisclosure   200    8     0     0     8    100.0%    0.0% +100.0%
inputval         20     4     0     0     6    100.0%    0.0% +100.0%
intoverflow      190    3     0     2     7     60.0%    0.0%  +60.0%
memsafety        119    8     0     0     12   100.0%    0.0% +100.0%
pathtraver       22     14    0     0     14   100.0%    0.0% +100.0%
redos            1333   5     0     0     5    100.0%    0.0% +100.0%
sqli             89     23    1     1     25    95.8%    3.8%  +92.0%
ssrf             918    9     0     0     13   100.0%    0.0% +100.0%
weakrand         330    7     0     0     9    100.0%    0.0% +100.0%
xss              79     11    0     0     11   100.0%    0.0% +100.0%
----------------------------------------------------------------------
TOTAL                   121   1     3     143   97.6%    0.7%  +96.9%

MACRO AVERAGE (OWASP):  96.6%   0.3%  +96.3%
```
