# PHP Benchmark Baseline -- TheAuditor

**Date**: 2026-03-30
**Version**: Post 11-team parallel fix session (round 5)
**Category-Averaged Score (OWASP)**: +96.6%
**Flat Score**: +95.1%

## Category Scores

| Category | CWE | TP | FP | FN | TN | TPR | FPR | Score |
|----------|-----|----|----|----|----|-----|-----|-------|
| cmdi | 78 | 10 | 0 | 2 | 12 | 83.3% | 0.0% | +83.3% |
| codeinj | 94 | 6 | 0 | 0 | 6 | 100.0% | 0.0% | +100.0% |
| csrf | 352 | 6 | 0 | 0 | 6 | 100.0% | 0.0% | +100.0% |
| deserial | 502 | 7 | 0 | 0 | 7 | 100.0% | 0.0% | +100.0% |
| extract | 621 | 5 | 0 | 0 | 5 | 100.0% | 0.0% | +100.0% |
| fileinclusion | 98 | 9 | 1 | 1 | 8 | 90.0% | 11.1% | +78.9% |
| fileupload | 434 | 6 | 0 | 0 | 6 | 100.0% | 0.0% | +100.0% |
| hardcodedcreds | 798 | 6 | 0 | 0 | 6 | 100.0% | 0.0% | +100.0% |
| headerinj | 113 | 3 | 0 | 1 | 4 | 75.0% | 0.0% | +75.0% |
| ldapi | 90 | 4 | 0 | 0 | 4 | 100.0% | 0.0% | +100.0% |
| massassign | 915 | 5 | 0 | 0 | 5 | 100.0% | 0.0% | +100.0% |
| pathtraver | 22 | 12 | 1 | 0 | 10 | 100.0% | 9.1% | +90.9% |
| redirect | 601 | 5 | 0 | 0 | 5 | 100.0% | 0.0% | +100.0% |
| securecookie | 614 | 4 | 0 | 0 | 4 | 100.0% | 0.0% | +100.0% |
| sqli | 89 | 26 | 0 | 2 | 28 | 92.9% | 0.0% | +92.9% |
| ssrf | 918 | 7 | 0 | 0 | 7 | 100.0% | 0.0% | +100.0% |
| ssti | 1336 | 5 | 0 | 0 | 5 | 100.0% | 0.0% | +100.0% |
| typejuggling | 697 | 7 | 0 | 0 | 7 | 100.0% | 0.0% | +100.0% |
| unsafereflect | 470 | 4 | 0 | 0 | 4 | 100.0% | 0.0% | +100.0% |
| variablevars | 627 | 4 | 0 | 0 | 4 | 100.0% | 0.0% | +100.0% |
| weakcipher | 327 | 3 | 0 | 0 | 3 | 100.0% | 0.0% | +100.0% |
| weakhash | 328 | 5 | 0 | 0 | 5 | 100.0% | 0.0% | +100.0% |
| weakrand | 330 | 5 | 0 | 0 | 5 | 100.0% | 0.0% | +100.0% |
| xss | 79 | 19 | 0 | 1 | 19 | 95.0% | 0.0% | +95.0% |
| xxe | 611 | 6 | 0 | 0 | 6 | 100.0% | 0.0% | +100.0% |

## Totals

| Metric | Value |
|--------|-------|
| Total test cases | 369 |
| Vulnerable (ground truth) | 186 |
| Safe (ground truth) | 183 |
| Total TP | 179 |
| Total FP | 2 |
| Total FN | 7 |
| Total TN | 181 |
| Categories scoring >0% | 25/25 |
| Categories at 100% | 19/25 |
| Categories at 0% | 0/25 |
