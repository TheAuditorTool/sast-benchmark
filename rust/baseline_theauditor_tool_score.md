# Rust Benchmark Scorecard: TheAuditor

**Tool:** TheAuditor v3.5
**Benchmark:** 268 test cases, 13 CWE categories, 4 frameworks
**Scoring:** SARIF pipeline (`convert_theauditor.py` + `score_sarif.py`)

---

## Current Score (2026-03-24, round 6)

```
Category         CWE    TP    FP    FN    TN      TPR     FPR   Score
----------------------------------------------------------------------
cmdi             78     14    0     0     16   100.0%    0.0% +100.0%
crypto           327    8     0     1     11    88.9%    0.0%  +88.9%
deser            502    4     0     2     6     66.7%    0.0%  +66.7%
infodisclosure   200    8     0     0     8    100.0%    0.0% +100.0%
inputval         20     4     0     0     6    100.0%    0.0% +100.0%
intoverflow      190    3     0     2     7     60.0%    0.0%  +60.0%
memsafety        119    1     0     7     12    12.5%    0.0%  +12.5%
pathtraver       22     14    0     0     14   100.0%    0.0% +100.0%
redos            1333   4     0     1     5     80.0%    0.0%  +80.0%
sqli             89     23    1     1     25    95.8%    3.8%  +92.0%
ssrf             918    9     0     0     13   100.0%    0.0% +100.0%
weakrand         330    7     0     0     9    100.0%    0.0% +100.0%
xss              79     11    0     0     11   100.0%    0.0% +100.0%
----------------------------------------------------------------------
TOTAL                   110   1     14    143   88.7%    0.7%  +88.0%

MACRO AVERAGE (OWASP):  84.9%   0.3%  +84.6%
```

Score = TPR - FPR (Youden's J). 0% = random guessing. +100% = perfect.
