# PHP Benchmark Baseline -- TheAuditor

**Date**: 2026-03-30
**Version**: Post 11-team parallel fix session (round 4)
**Category-Averaged Score (OWASP)**: +71.7%
**Flat Score**: +74.6%

## Category Scores

| Category | CWE | TP | FP | FN | TN | TPR | FPR | Score |
|----------|-----|----|----|----|----|-----|-----|-------|
| cmdi | 78 | 9 | 1 | 3 | 11 | 75.0% | 8.3% | +66.7% |
| codeinj | 94 | 6 | 2 | 0 | 4 | 100.0% | 33.3% | +66.7% |
| csrf | 352 | 6 | 0 | 0 | 6 | 100.0% | 0.0% | +100.0% |
| deserial | 502 | 6 | 2 | 1 | 5 | 85.7% | 28.6% | +57.1% |
| extract | 621 | 5 | 0 | 0 | 5 | 100.0% | 0.0% | +100.0% |
| fileinclusion | 98 | 7 | 1 | 3 | 8 | 70.0% | 11.1% | +58.9% |
| fileupload | 434 | 6 | 0 | 0 | 6 | 100.0% | 0.0% | +100.0% |
| hardcodedcreds | 798 | 6 | 0 | 0 | 6 | 100.0% | 0.0% | +100.0% |
| headerinj | 113 | 2 | 0 | 2 | 4 | 50.0% | 0.0% | +50.0% |
| ldapi | 90 | 4 | 1 | 0 | 3 | 100.0% | 25.0% | +75.0% |
| massassign | 915 | 5 | 0 | 0 | 5 | 100.0% | 0.0% | +100.0% |
| pathtraver | 22 | 12 | 1 | 0 | 10 | 100.0% | 9.1% | +90.9% |
| redirect | 601 | 2 | 0 | 3 | 5 | 40.0% | 0.0% | +40.0% |
| securecookie | 614 | 4 | 0 | 0 | 4 | 100.0% | 0.0% | +100.0% |
| sqli | 89 | 28 | 0 | 0 | 28 | 100.0% | 0.0% | +100.0% |
| ssrf | 918 | 7 | 7 | 0 | 0 | 100.0% | 100.0% | +0.0% |
| ssti | 1336 | 0 | 0 | 5 | 5 | 0.0% | 0.0% | +0.0% |
| typejuggling | 697 | 7 | 0 | 0 | 7 | 100.0% | 0.0% | +100.0% |
| unsafereflect | 470 | 4 | 0 | 0 | 4 | 100.0% | 0.0% | +100.0% |
| variablevars | 627 | 0 | 0 | 4 | 4 | 0.0% | 0.0% | +0.0% |
| weakcipher | 327 | 3 | 0 | 0 | 3 | 100.0% | 0.0% | +100.0% |
| weakhash | 328 | 5 | 0 | 0 | 5 | 100.0% | 0.0% | +100.0% |
| weakrand | 330 | 2 | 0 | 3 | 5 | 40.0% | 0.0% | +40.0% |
| xss | 79 | 14 | 1 | 6 | 18 | 70.0% | 5.3% | +64.7% |
| xxe | 611 | 5 | 0 | 1 | 6 | 83.3% | 0.0% | +83.3% |

## Totals

| Metric | Value |
|--------|-------|
| Total test cases | 369 |
| Vulnerable (ground truth) | 186 |
| Safe (ground truth) | 183 |
| Total TP | 155 |
| Total FP | 16 |
| Total FN | 31 |
| Total TN | 167 |
| Categories scoring >0% | 21/25 |
| Categories at 100% | 12/25 |
| Categories at 0% | 4/25 |
