# Baseline Score: TheAuditor (Chain Detection Benchmark)

**Tool:** TheAuditor (java-support branch)
**Benchmark:** Chain Detection Benchmark v0.1.0 (16 test cases, 8 scenarios, 4 categories)
**Date:** 2026-04-01
**Configuration:** `aud full --offline` (default rules, no tuning)

---

## Why We Publish This

This is a zero score. We publish it anyway because it establishes the starting line. The chain detection benchmark tests a capability that does not yet exist in TheAuditor -- correlating multiple findings across files into compound exploit chains. Every FN below is a detection gap the chain tracer is being built to close.

---

## Scorecard

```
Category               CWE    TP    FP    FN    TN      TPR     FPR   Score
--------------------------------------------------------------------------------
compound_injection     79     1     0     1     2     50.0%    0.0%  +50.0%
multi_stage            434    0     0     2     2      0.0%    0.0%   +0.0%
ssrf_pivot             918    0     0     2     2      0.0%    0.0%   +0.0%
unauth_injection       89     1     1     1     1     50.0%   50.0%   +0.0%
--------------------------------------------------------------------------------
OVERALL                       2     1     6     7     25.0%   12.5%  +12.5%
```

**Category-Averaged Score: +12.5%** | **Flat Score: +12.5%** (Youden's J = TPR - FPR)

Scored via unified pipeline: `convert_theauditor.py` -> `score_sarif.py`

---

## What The Numbers Mean

- **2 True Positives**: Chain endpoints where the tool detected the correct CWE within annotation range
- **7 True Negatives**: Mitigated chains correctly ignored
- **6 False Negatives**: Exploitable chains missed
- **1 False Positive**: Mitigated chain falsely flagged (unauth_injection safe variant)

The tool detects individual vulnerabilities (SQLi, XSS) that happen to overlap with chain endpoint annotations. This is not true chain-aware detection -- the tool does not correlate findings across files. The +12.5% score reflects incidental CWE matches at chain endpoints, not compositional reasoning. True chain detection requires rules that connect auth gaps to injection sinks.

---

## False Negatives (6 of 8 Exploitable Chains Missed)

| Scenario | Category | Chain Logic | Why Missed |
|----------|----------|-------------|------------|
| auth_bypass_to_sqli | unauth_injection | Missing auth on route + SQLi = unauthenticated SQLi | No cross-file auth-to-sink correlation |
| auth_bypass_to_cmdi | unauth_injection | Timing-attackable token + CmdI = unauthenticated RCE | No auth weakness + injection composition |
| ssrf_to_cloud_metadata | ssrf_pivot | SSRF in proxy + cloud metadata = credential theft | No SSRF target reachability analysis |
| ssrf_to_internal_admin | ssrf_pivot | SSRF in webhook + internal admin API = admin access | No internal service trust boundary modeling |
| sqli_to_stored_xss | compound_injection | SQLi stores payload + template renders unescaped = stored XSS | No second-order injection tracking |
| deser_to_rce | compound_injection | Untrusted pickle.loads = arbitrary code execution | No unsafe deserialization chain rule |
| file_upload_to_rce | multi_stage | Upload without ext check + plugin runner = RCE | No upload-to-execution path analysis |
| log_injection_to_xss | multi_stage | Unsanitized log writes + raw log rendering = admin XSS | No cross-context taint (log write -> log read) |

---

## False Positives

None. The tool does not produce chain-category findings, so it cannot false-positive on mitigated chains. This is the trivial case -- a tool that reports nothing has 0% FPR by definition.

---

## What Needs To Be Built

The chain tracer needs to implement cross-file finding correlation. The individual findings already exist in the database (the tool detects the SQLi, the CmdI, the SSRF individually). What's missing is the logic that says "this SQLi is behind an unauthenticated route, therefore the compound severity is critical."

**Required engine capabilities:**

1. **Auth-to-sink correlation**: Connect missing/weak auth findings on routes to injection findings on the handlers those routes reach. Output: `unauth_injection` chain finding.

2. **SSRF target reachability**: When SSRF is detected, analyze what internal services the SSRF can reach. If an internal service has no auth (trusts network boundary), output: `ssrf_pivot` chain finding.

3. **Second-order injection tracking**: Track tainted data written to storage (DB, logs, files) and flag when that data is read and rendered without sanitization in a different context. Output: `compound_injection` chain finding.

4. **Upload-to-execution path**: Connect file upload endpoints to code that loads/executes uploaded files. Output: `multi_stage` chain finding.

**Scoring note:** The chain_benchmark.py scoring script checks three DB tracks:
- `chain_findings` table (dedicated chain output -- does not exist yet)
- `pattern_findings` with chain-category rule IDs (e.g., `chain-unauth-sqli`)
- `resolved_flow_audit` with chain-category vulnerability types (e.g., `Unauthenticated Injection`)

Any of these three tracks will register as a chain detection. Implement whichever fits the engine architecture best.
