# PHP Benchmark Baseline -- TheAuditor

**Note**: This baseline was scored against v0.1.0 (369 tests). Re-scoring against v0.2.0 (562 tests) is pending.

**Date**: 2026-04-01
**Version**: TheAuditor (current) -- PERFECT SCORE (v0.1.0)
**Category-Averaged Score (OWASP)**: +100.0%
**Flat Score**: +100.0%

## Category Scores

| Category | CWE | TP | FP | FN | TN | TPR | FPR | Score |
|----------|-----|----|----|----|----|-----|-----|-------|
| cmdi | 78 | 12 | 0 | 0 | 12 | 100.0% | 0.0% | +100.0% |
| codeinj | 94 | 6 | 0 | 0 | 6 | 100.0% | 0.0% | +100.0% |
| csrf | 352 | 6 | 0 | 0 | 6 | 100.0% | 0.0% | +100.0% |
| deserial | 502 | 7 | 0 | 0 | 7 | 100.0% | 0.0% | +100.0% |
| extract | 621 | 5 | 0 | 0 | 5 | 100.0% | 0.0% | +100.0% |
| fileinclusion | 98 | 10 | 0 | 0 | 9 | 100.0% | 0.0% | +100.0% |
| fileupload | 434 | 6 | 0 | 0 | 6 | 100.0% | 0.0% | +100.0% |
| hardcodedcreds | 798 | 6 | 0 | 0 | 6 | 100.0% | 0.0% | +100.0% |
| headerinj | 113 | 4 | 0 | 0 | 4 | 100.0% | 0.0% | +100.0% |
| ldapi | 90 | 4 | 0 | 0 | 4 | 100.0% | 0.0% | +100.0% |
| massassign | 915 | 5 | 0 | 0 | 5 | 100.0% | 0.0% | +100.0% |
| pathtraver | 22 | 12 | 0 | 0 | 11 | 100.0% | 0.0% | +100.0% |
| redirect | 601 | 5 | 0 | 0 | 5 | 100.0% | 0.0% | +100.0% |
| securecookie | 614 | 4 | 0 | 0 | 4 | 100.0% | 0.0% | +100.0% |
| sqli | 89 | 28 | 0 | 0 | 28 | 100.0% | 0.0% | +100.0% |
| ssrf | 918 | 7 | 0 | 0 | 7 | 100.0% | 0.0% | +100.0% |
| ssti | 1336 | 5 | 0 | 0 | 5 | 100.0% | 0.0% | +100.0% |
| typejuggling | 697 | 7 | 0 | 0 | 7 | 100.0% | 0.0% | +100.0% |
| unsafereflect | 470 | 4 | 0 | 0 | 4 | 100.0% | 0.0% | +100.0% |
| variablevars | 627 | 4 | 0 | 0 | 4 | 100.0% | 0.0% | +100.0% |
| weakcipher | 327 | 3 | 0 | 0 | 3 | 100.0% | 0.0% | +100.0% |
| weakhash | 328 | 5 | 0 | 0 | 5 | 100.0% | 0.0% | +100.0% |
| weakrand | 330 | 5 | 0 | 0 | 5 | 100.0% | 0.0% | +100.0% |
| xss | 79 | 20 | 0 | 0 | 19 | 100.0% | 0.0% | +100.0% |
| xxe | 611 | 6 | 0 | 0 | 6 | 100.0% | 0.0% | +100.0% |

## Totals

| Metric | Value |
|--------|-------|
| Total test cases | 369 |
| Vulnerable (ground truth) | 186 |
| Safe (ground truth) | 183 |
| Total TP | 186 |
| Total FP | 0 |
| Total FN | 0 |
| Total TN | 183 |
| Categories scoring >0% | 25/25 |
| Categories at 100% | 25/25 |
| Categories at 0% | 0/25 |

## Changes from Previous Baseline (2026-03-30)

| Category | Previous | Current | Delta |
|----------|----------|---------|-------|
| cmdi | +83.3% | +100.0% | +16.7% |
| fileinclusion | +78.9% | +100.0% | +21.1% |
| headerinj | +75.0% | +100.0% | +25.0% |
| pathtraver | +90.9% | +100.0% | +9.1% |
| sqli | +92.9% | +100.0% | +7.1% |
| xss | +95.0% | +100.0% | +5.0% |
| **OWASP** | **+96.6%** | **+100.0%** | **+3.4%** |
| **Flat** | **+95.1%** | **+100.0%** | **+4.9%** |

## What Changed

- Improved sanitizer recognition for header injection (urlencode, rawurlencode, str_replace)
- Fixed constant array allowlist detection for file operations and inclusions
- Fixed argument index handling for legacy mysql_* procedural functions
- Improved disambiguation between PDO::exec (SQL) and exec() (shell command)
- Added property access tracing for stored XSS through object iteration patterns
