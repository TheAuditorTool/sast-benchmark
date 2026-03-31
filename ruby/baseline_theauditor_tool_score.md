# Ruby Benchmark Baseline -- TheAuditor

**Date**: 2026-03-31
**Version**: TheAuditor (current java-support branch)
**Category-Averaged Score (OWASP)**: +4.0%
**Flat Score**: +2.5%

## Category Scores

| Category | CWE | TP | FP | FN | TN | TPR | FPR | Score |
|----------|-----|----|----|----|----|----|-----|-------|
| cmdi | 78 | 0 | 0 | 12 | 12 | 0.0% | 0.0% | +0.0% |
| codeinj | 94 | 0 | 0 | 6 | 6 | 0.0% | 0.0% | +0.0% |
| csrf | 352 | 0 | 0 | 6 | 6 | 0.0% | 0.0% | +0.0% |
| deserial | 502 | 0 | 0 | 8 | 8 | 0.0% | 0.0% | +0.0% |
| dynmethod | 94 | 0 | 0 | 4 | 4 | 0.0% | 0.0% | +0.0% |
| fileinclusion | 98 | 0 | 0 | 5 | 5 | 0.0% | 0.0% | +0.0% |
| fileupload | 434 | 0 | 0 | 6 | 6 | 0.0% | 0.0% | +0.0% |
| hardcodedcreds | 798 | 0 | 0 | 6 | 6 | 0.0% | 0.0% | +0.0% |
| headerinj | 113 | 0 | 0 | 4 | 4 | 0.0% | 0.0% | +0.0% |
| ldapi | 90 | 0 | 0 | 3 | 3 | 0.0% | 0.0% | +0.0% |
| loginjection | 117 | 0 | 0 | 3 | 3 | 0.0% | 0.0% | +0.0% |
| massassign | 915 | 0 | 0 | 7 | 7 | 0.0% | 0.0% | +0.0% |
| pathtraver | 22 | 0 | 0 | 9 | 9 | 0.0% | 0.0% | +0.0% |
| redirect | 601 | 0 | 0 | 6 | 6 | 0.0% | 0.0% | +0.0% |
| regex | 1333 | 0 | 0 | 3 | 3 | 0.0% | 0.0% | +0.0% |
| securecookie | 614 | 0 | 0 | 5 | 5 | 0.0% | 0.0% | +0.0% |
| sqli | 89 | 0 | 0 | 19 | 19 | 0.0% | 0.0% | +0.0% |
| ssrf | 918 | 0 | 0 | 6 | 6 | 0.0% | 0.0% | +0.0% |
| ssti | 1336 | 0 | 0 | 6 | 6 | 0.0% | 0.0% | +0.0% |
| unsafereflect | 470 | 0 | 0 | 4 | 4 | 0.0% | 0.0% | +0.0% |
| weakcipher | 327 | 0 | 0 | 3 | 3 | 0.0% | 0.0% | +0.0% |
| weakhash | 328 | 0 | 0 | 6 | 6 | 0.0% | 0.0% | +0.0% |
| weakrand | 330 | 4 | 0 | 0 | 4 | 100.0% | 0.0% | +100.0% |
| xss | 79 | 0 | 0 | 14 | 14 | 0.0% | 0.0% | +0.0% |
| xxe | 611 | 0 | 0 | 4 | 4 | 0.0% | 0.0% | +0.0% |

## Summary

- Total test cases: 318 (159 vulnerable, 159 safe -- 50/50 balance)
- Tests with findings: 4 (all TP, zero FP)
- Only `weakrand` (CWE-330) detected: 4/4 TP, 0 FP -- perfect on that category
- 24 of 25 categories: zero detection (0 TP, 0 FP)

## SARIF Findings Detail (9 total, 4 matched to test cases)

| ruleId | File | Line | Test Case Key | Matched? |
|--------|------|------|---------------|----------|
| 915 | testcode/shared.rb:80 | 80 | NO_KEY | No (outside annotation range) |
| 327 | apps/rack_app/auth.rb:37 | 37 | rk_weakhash_md5 | No (CWE 327 != expected 328) |
| 327 | testcode/weakhash_001.rb:7 | 7 | ruby_weakhash_md5 | No (CWE 327 != expected 328) |
| 327 | testcode/weakhash_003.rb:7 | 7 | ruby_weakhash_sha1 | No (CWE 327 != expected 328) |
| 330 | testcode/weakrand_003.rb:6 | 6 | ruby_weakrand_random_rand | Yes (TP) |
| 330 | apps/rack_app/auth.rb:55 | 55 | rk_weakrand_session | Yes (TP) |
| 330 | testcode/weakrand_001.rb:6 | 6 | ruby_weakrand_rand | Yes (TP) |
| 330 | testcode/weakrand_005.rb:7 | 7 | ruby_weakrand_srand | Yes (TP) |

## Notes

- **CWE mismatch on weakhash**: TheAuditor reports CWE-327 (Weak Cryptography) but benchmark expects CWE-328 (Weak Hash). 3 correct detections not scoring because of CWE number mismatch. Fixing this mapping would add 3 TP to weakhash category.
- **Zero taint flow detections**: No sqli, cmdi, xss, ssrf, pathtraver, or any flow-based category detected. Ruby taint pipeline needs investigation.
- **Zero FP**: Clean on false positives -- no incorrect findings.
