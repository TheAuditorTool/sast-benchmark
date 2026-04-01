# Ruby Benchmark Baseline -- TheAuditor

**Date**: 2026-04-01
**Version**: TheAuditor (current java-support branch)
**Category-Averaged Score (OWASP)**: +99.1%
**Flat Score**: +98.7%

## Category Scores

| Category       | CWE  |  TP |  FP |  FN |  TN |    TPR |    FPR |   Score |
|----------------|------|----:|----:|----:|----:|-------:|-------:|--------:|
| cmdi           | 78   |  12 |   0 |   0 |  12 | 100.0% |  0.0%  | +100.0% |
| codeinj        | 94   |   6 |   0 |   0 |   6 | 100.0% |  0.0%  | +100.0% |
| csrf           | 352  |   6 |   0 |   0 |   6 | 100.0% |  0.0%  | +100.0% |
| deserial       | 502  |   8 |   0 |   0 |   8 | 100.0% |  0.0%  | +100.0% |
| dynmethod      | 94   |   4 |   0 |   0 |   4 | 100.0% |  0.0%  | +100.0% |
| fileinclusion  | 98   |   5 |   0 |   0 |   5 | 100.0% |  0.0%  | +100.0% |
| fileupload     | 434  |   6 |   0 |   0 |   6 | 100.0% |  0.0%  | +100.0% |
| hardcodedcreds | 798  |   6 |   0 |   0 |   6 | 100.0% |  0.0%  | +100.0% |
| headerinj      | 113  |   4 |   0 |   0 |   4 | 100.0% |  0.0%  | +100.0% |
| ldapi          | 90   |   3 |   0 |   0 |   3 | 100.0% |  0.0%  | +100.0% |
| loginjection   | 117  |   3 |   0 |   0 |   3 | 100.0% |  0.0%  | +100.0% |
| massassign     | 915  |   7 |   0 |   0 |   7 | 100.0% |  0.0%  | +100.0% |
| pathtraver     | 22   |   9 |   0 |   0 |   9 | 100.0% |  0.0%  | +100.0% |
| redirect       | 601  |   6 |   1 |   0 |   5 | 100.0% | 16.7%  | +83.3%  |
| regex          | 1333 |   3 |   0 |   0 |   3 | 100.0% |  0.0%  | +100.0% |
| securecookie   | 614  |   5 |   0 |   0 |   5 | 100.0% |  0.0%  | +100.0% |
| sqli           | 89   |  19 |   1 |   0 |  18 | 100.0% |  5.3%  | +94.7%  |
| ssrf           | 918  |   6 |   0 |   0 |   6 | 100.0% |  0.0%  | +100.0% |
| ssti           | 1336 |   6 |   0 |   0 |   6 | 100.0% |  0.0%  | +100.0% |
| unsafereflect  | 470  |   4 |   0 |   0 |   4 | 100.0% |  0.0%  | +100.0% |
| weakcipher     | 327  |   3 |   0 |   0 |   3 | 100.0% |  0.0%  | +100.0% |
| weakhash       | 328  |   6 |   0 |   0 |   6 | 100.0% |  0.0%  | +100.0% |
| weakrand       | 330  |   4 |   0 |   0 |   4 | 100.0% |  0.0%  | +100.0% |
| xss            | 79   |  14 |   0 |   0 |  14 | 100.0% |  0.0%  | +100.0% |
| xxe            | 611  |   4 |   0 |   0 |   4 | 100.0% |  0.0%  | +100.0% |

## Summary

- Total test cases: 318 (159 vulnerable, 159 safe -- 50/50 balance)
- Tests with findings: 161
- 23 of 25 categories at +100% (perfect)
- 2 categories below +100%: redirect (+83.3%, 1 FP), sqli (+94.7%, 1 FP)
- 100% TPR across ALL 25 categories (zero false negatives)
- Only 2 false positives remaining in the entire benchmark
