# Baseline Scorecard: TheAuditor v3.5 vs Go Benchmark v0.3

> **Tool**: TheAuditor v3.5
> **Benchmark**: Go SAST Benchmark v0.3 (424 test cases, 16 CWE categories)
> **Date**: 2026-03-20
> **Command**: `aud full --offline`
> **Tuning**: None. Out-of-the-box results with no benchmark-specific adjustments.

## Why We Publish This

OWASP publishes every tool's score against BenchmarkJava -- Checkmarx, Fortify, Veracode, SonarQube, all of them. Honest scores build credibility. Hidden scores destroy it.

We built this benchmark and we ran our own tool against it. Here are the results, untuned, with every false negative and false positive visible. These gaps are our public roadmap.

## Scorecard

```
Category         CWE    TP    FP    FN    TN      TPR     FPR   Score
---------------------------------------------------------------------------
cmdi             78     23    19    7     11      76.7%   63.3%  +13.3%
deserial         502    1     1     3     3       25.0%   25.0%   +0.0%
ldapi            90     4     4     0     0      100.0%  100.0%   +0.0%
loginjection     117    1     0     3     4       25.0%    0.0%  +25.0%
nosql            943    4     4     0     0      100.0%  100.0%   +0.0%
pathtraver       22     22    20    3     10      88.0%   66.7%  +21.3%
redirect         601    0     0     8     8        0.0%    0.0%   +0.0%
securecookie     614    0     0     8     8        0.0%    0.0%   +0.0%
sqli             89     49    21    16    37      75.4%   36.2%  +39.2%
ssrf             918    0     0     10    10       0.0%    0.0%   +0.0%
tlsverify        295    3     0     2     5       60.0%    0.0%  +60.0%
trustbound       501    0     0     4     4        0.0%    0.0%   +0.0%
weakcipher       327    4     0     4     8       50.0%    0.0%  +50.0%
weakhash         328    10    1     0     9      100.0%   10.0%  +90.0%
weakrand         330    8     2     2     8       80.0%   20.0%  +60.0%
xss              79     6     1     7     14      46.2%    6.7%  +39.5%
---------------------------------------------------------------------------
OVERALL                 135   73    77    139     63.7%   34.4%  +29.2%
```

**Flat Score: +29.2%** (Youden's J = TPR - FPR)

**Category-Averaged Score: +24.9%** (OWASP standard -- each category weighted equally)

Note: The category-averaged score is lower because categories where TheAuditor has 0% detection (redirect, securecookie, ssrf, trustbound) each count as a full 0% TPR in the average rather than being diluted by larger categories. This is a more honest representation of tool capability across all vulnerability classes. The v0.3.1 benchmark adds 5 new CWE categories (hardcodedcreds, authnfailure, authzfailure, csrf, codeinj) which are not reflected in this baseline -- a re-run is needed for a complete v0.3.1 scorecard.

## What the Scores Mean

**Strong detection (Score > +50%)**:
- **weakhash (+90.0%)**: Near-perfect. Detects md5/sha1 in security contexts, correctly ignores md5 for cache keys and sha256/sha512/bcrypt. 10/10 TP, 9/10 TN.
- **tlsverify (+60.0%)**: Catches InsecureSkipVerify and weak TLS versions. Zero false positives. Misses VerifyPeerCertificate-always-nil and MinVersion:0.
- **weakrand (+60.0%)**: Detects math/rand in security contexts (session tokens, API keys). Some FP on non-security usage (display shuffle).
- **weakcipher (+50.0%)**: Catches DES and RC4. Misses AES-ECB, AES-CBC-without-auth, XOR cipher, ROT13.

**Moderate detection (Score +20-50%)**:
- **sqli (+39.2%)**: Detects 49/65 vulnerable patterns (75.4% TPR) but 21 false positives on safe patterns (36.2% FPR). The discrimination patterns (dead-code, variable overwrite, map confusion) are working -- they expose FP weaknesses in taint analysis.
- **xss (+39.5%)**: Catches template.HTML/JS bypasses and direct response writes. Misses text/template, SSE, WebSocket, FuncMap bypass patterns.
- **loginjection (+25.0%)**: Early stage. Catches log.Printf with user input. Misses slog, file-based logging, structured logging.
- **pathtraver (+21.3%)**: High recall (88%) but high FPR (66.7%). Detects most traversal patterns but false-alarms on filepath.Base, filepath.Clean+HasPrefix, and integer-based paths.
- **cmdi (+13.3%)**: Same pattern as pathtraver -- good recall (76.7%) but too many false positives (63.3%). Flags safe patterns where command is hardcoded and user only controls args.

**No discrimination (Score = 0%)**:
- **ldapi, nosql**: 100% TPR but 100% FPR. Flags everything -- both vulnerable and safe patterns. No taint flow discrimination, just pattern matching.
- **deserial**: Catches gob.Decode to interface{} but also flags typed struct deserialization. Needs typed-vs-untyped discrimination.

**No detection (Score = 0%, TPR = 0%)**:
- **redirect**: No rule fires for http.Redirect with user input.
- **securecookie**: No rule checks http.Cookie Secure/HttpOnly flags.
- **ssrf**: No taint-confirmed SSRF flows in standalone test files (taint engine finds SSRF in reference apps but not in single-file tests).
- **trustbound**: No rule for session.Values with user input.

## Analysis

**135 True Positives**: TheAuditor correctly identifies 63.7% of vulnerable Go patterns. Strongest in structural detection (crypto, hashing, TLS) where API usage alone determines vulnerability.

**77 False Negatives**: 36.3% of vulnerable patterns missed. Concentrated in:
- Categories with zero rules (redirect, securecookie, ssrf standalone, trustbound)
- Complex flow patterns (multi-hop, cross-file, closure, channel propagation)
- Non-standard sink patterns (text/template for SQL, bytes.Buffer, SSE, WebSocket)

**73 False Positives**: 34.4% of safe patterns incorrectly flagged. Concentrated in:
- cmdi/pathtraver: hardcoded command with user args flagged as injection
- sqli: discrimination patterns (dead-code, variable overwrite) not tracked through data flow
- ldapi/nosql: no TP/FP discrimination at all (pattern-only, no taint)

**139 True Negatives**: 65.6% of safe patterns correctly ignored. Strongest in crypto (sha256, bcrypt, AES-GCM correctly recognized as safe) and structural categories.

## Improvement Roadmap

Each FN and FP is a trackable improvement:

| Priority | Action | Tests Affected | Expected Score Impact |
|----------|--------|---------------|----------------------|
| 1 | Add securecookie rule (http.Cookie flags) | 16 tests | +securecookie from 0% |
| 2 | Add redirect rule (http.Redirect taint) | 16 tests | +redirect from 0% |
| 3 | Fix cmdi/pathtraver FPR (hardcoded cmd = safe) | ~40 tests | cmdi +30%, pathtraver +25% |
| 4 | Add SSRF standalone taint flow | 20 tests | +ssrf from 0% |
| 5 | Add ldapi/nosql taint discrimination | 16 tests | ldapi/nosql from 0% to +50%+ |
| 6 | Fix sqli discrimination (dead-code, overwrite) | ~20 tests | sqli from +39% to +60%+ |

## How to Reproduce

```bash
cd C:\Users\santa\Desktop\open_tests\gorustbash_benchmark\go
aud full --offline
# Then run the scoring script from go_benchmark.md
```
