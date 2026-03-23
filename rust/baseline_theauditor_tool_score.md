# Rust Benchmark Scorecard: TheAuditor

**Tool:** TheAuditor v3.5
**Benchmark:** 268 test cases, 13 CWE categories, 4 frameworks
**Scoring:** SARIF pipeline (`convert_theauditor.py` + `score_sarif.py`)

---

## Current Score (2026-03-23, round 5)

```
Category         CWE    TP    FP    FN    TN      TPR     FPR   Score
----------------------------------------------------------------------
cmdi             78     14    2     0     14   100.0%   12.5%  +87.5%
crypto           327    9     0     0     11   100.0%    0.0% +100.0%
deser            502    6     0     0     6    100.0%    0.0% +100.0%
infodisclosure   200    8     0     0     8    100.0%    0.0% +100.0%
inputval         20     4     0     0     6    100.0%    0.0% +100.0%
intoverflow      190    5     0     0     7    100.0%    0.0% +100.0%
memsafety        119    8     0     0     12   100.0%    0.0% +100.0%
pathtraver       22     4     0     10    14    28.6%    0.0%  +28.6%
redos            1333   5     0     0     5    100.0%    0.0% +100.0%
sqli             89     9     1     15    25    37.5%    3.8%  +33.7%
ssrf             918    9     0     0     13   100.0%    0.0% +100.0%
weakrand         330    7     0     0     9    100.0%    0.0% +100.0%
xss              79     11    0     0     11   100.0%    0.0% +100.0%
----------------------------------------------------------------------
TOTAL                   99    3     25    141   79.8%    2.1%  +77.8%

MACRO AVERAGE (OWASP):  89.7%   1.3%  +88.4%
```

Score = TPR - FPR (Youden's J). 0% = random guessing. +100% = perfect.

---

## Previous Score (2026-03-23, round 4)

```
Category         CWE    TP    FP    FN    TN      TPR     FPR   Score
----------------------------------------------------------------------
cmdi             78     13    1     1     15    92.9%    6.2%  +86.6%
crypto           327    9     9     0     2    100.0%   81.8%  +18.2%
deser            502    6     0     0     6    100.0%    0.0% +100.0%
infodisclosure   200    8     0     0     8    100.0%    0.0% +100.0%
inputval         20     4     0     0     6    100.0%    0.0% +100.0%
intoverflow      190    5     2     0     5    100.0%   28.6%  +71.4%
memsafety        119    8     6     0     6    100.0%   50.0%  +50.0%
pathtraver       22     14    1     0     13   100.0%    7.1%  +92.9%
redos            1333   5     4     0     1    100.0%   80.0%  +20.0%
sqli             89     23    13    1     13    95.8%   50.0%  +45.8%
ssrf             918    7     1     2     12    77.8%    7.7%  +70.1%
weakrand         330    7     4     0     5    100.0%   44.4%  +55.6%
xss              79     11    0     0     11   100.0%    0.0% +100.0%
----------------------------------------------------------------------
TOTAL                   120   41    4     103   96.8%   28.5%  +68.3%

MACRO AVERAGE (OWASP):  97.4%  27.4%  +70.0%
```

---

## Previous Score (2026-03-23, round 3)

```
Category         CWE    TP    FP    FN    TN      TPR     FPR   Score
----------------------------------------------------------------------
cmdi             78     11    6     3     10    78.6%   37.5%  +41.1%
crypto           327    9     9     0     2    100.0%   81.8%  +18.2%
deser            502    5     0     1     6     83.3%    0.0%  +83.3%
infodisclosure   200    8     0     0     8    100.0%    0.0% +100.0%
inputval         20     4     0     0     6    100.0%    0.0% +100.0%
intoverflow      190    5     2     0     5    100.0%   28.6%  +71.4%
memsafety        119    8     6     0     6    100.0%   50.0%  +50.0%
pathtraver       22     14    4     0     10   100.0%   28.6%  +71.4%
redos            1333   1     1     4     4     20.0%   20.0%   +0.0%
sqli             89     22    15    2     11    91.7%   57.7%  +34.0%
ssrf             918    7     1     2     12    77.8%    7.7%  +70.1%
weakrand         330    7     4     0     5    100.0%   44.4%  +55.6%
xss              79     11    0     0     11   100.0%    0.0% +100.0%
----------------------------------------------------------------------
TOTAL                   112   48    12    96    90.3%   33.3%  +57.0%

MACRO AVERAGE (OWASP):  88.6%  27.4%  +61.2%
```

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
| 2026-03-23 | SARIF | +42.2% | +32.3% | 111 | 89 | 7 | 55 | Round 2, 11-team parallel fixes |
| 2026-03-23 | SARIF | +61.2% | +57.0% | 112 | 48 | 12 | 96 | Round 3, cross-file edges, warp support, sanitizer fixes |
| 2026-03-23 | SARIF | +70.0% | +68.3% | 120 | 41 | 4 | 103 | Round 4, extract_base_variable fix, match_allowlist purge, EXIT::REGEX wiring |
| 2026-03-23 | SARIF | +88.4% | +77.8% | 99 | 3 | 25 | 141 | Round 5, category-aware scorer (OWASP standard), 10/13 categories at +100%, FPR 1.3% |
