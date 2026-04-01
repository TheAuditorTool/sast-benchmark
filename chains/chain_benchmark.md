# Chain Detection Benchmark

**Created:** 2026-03-31 | **Version:** v0.1.0
**Test Cases:** 16 (8 exploitable chains / 8 mitigated chains) across 4 categories
**Scenarios:** 8 multi-file applications in Python (Flask)

---

## Purpose

This benchmark tests a capability that no existing SAST benchmark measures: **can your tool correlate multiple individual findings across files into compound exploit paths whose severity exceeds any individual component?**

Traditional SAST benchmarks (OWASP Java, this suite's Go/Rust/Bash/PHP/Ruby benchmarks) test whether a tool can detect a single vulnerability in isolation. The adversarial evasion benchmark tests whether a tool can detect intentional concealment. This benchmark tests a third, distinct capability: **compositional reasoning**.

The question is not "can you find the SQL injection?" or "can you find the missing auth?" -- it is "can you determine that the missing auth on route X *enables* the SQL injection on endpoint Y to be exploitable by unauthenticated attackers, making the compound risk critical rather than medium?"

---

## Why This Matters

Real-world exploits are almost never single-finding affairs. The Capital One breach (2019) required SSRF + cloud metadata access + overly permissive IAM roles. Log4Shell (2021) required JNDI injection + attacker-controlled LDAP + classpath gadgets. The Equifax breach (2017) required Apache Struts RCE + unpatched server + unrestricted network access to databases.

SAST tools that report each component individually produce finding lists that are hundreds of items long, each classified as "medium" severity. Security teams triage by severity, so medium findings get deprioritized. The compound critical path hides in plain sight.

A tool that can identify compound exploit chains -- "these three medium findings, when combined, create a critical unauthenticated RCE path" -- provides fundamentally more valuable output than one that reports them separately.

---

## How It Works

### Scenario Structure

Each chain scenario is a small multi-file Flask application with two variants:

```
scenarios/<name>/
  vuln/       # Exploitable chain -- all links connected
    app.py
    middleware.py
    routes.py
  safe/       # Mitigated chain -- exactly one link broken
    app.py
    middleware.py
    routes.py
```

The **vuln** variant has an exploitable compound vulnerability. The **safe** variant breaks exactly one link in the chain, making the compound path non-exploitable. All other files are byte-identical between variants.

### What Counts as Chain Detection

A finding counts as a chain detection ONLY if it maps to a **chain-specific category** (unauth_injection, ssrf_pivot, compound_injection, multi_stage). Standard individual SAST findings (sqli, cmdi, xss, etc.) do NOT count.

This is the critical distinction. A tool that reports:
- "SQL injection at app.py:25" in both vuln and safe variants -- **NOT chain detection** (individual finding)
- "Unauthenticated SQL injection via missing auth on /admin/search" in the vuln variant only -- **chain detection** (compound finding)

The tool must demonstrate that it understands the *chain relationship*, not just the individual components.

### Scoring

Same Youden's J formula as all other benchmarks:

```
Score = TPR - FPR
TPR = TP / (TP + FN)
FPR = FP / (FP + TN)
```

| Score | Meaning |
|-------|---------|
| +100% | Detects all exploitable chains, no false alarms on mitigated chains |
| 0% | Random guessing |
| -100% | Flags mitigated chains, misses exploitable chains |

Scoring uses the same unified pipeline as all other benchmarks:

```bash
python ../scripts/convert_theauditor.py .pf/repo_index.db
python ../scripts/score_sarif.py theauditor.sarif expectedresults-0.1.0.csv
```

---

## Categories

### 1. unauth_injection -- Authentication Bypass Enabling Injection

Missing or bypassable authentication that exposes injection vulnerabilities to unauthenticated attackers. The injection alone is medium (requires auth). The auth gap alone is medium. Together: critical unauthenticated injection.

**Scenarios:**
- **auth_bypass_to_sqli**: Missing auth middleware on admin search route + SQL injection
- **auth_bypass_to_cmdi**: Timing-attackable token comparison + command injection

### 2. ssrf_pivot -- SSRF Used to Reach Internal Services

Server-side request forgery in a public-facing endpoint used to reach internal services that trust the network boundary. The SSRF alone is medium (hits external sites). The internal service alone is accepted risk (behind firewall). Together: critical internal service compromise.

**Scenarios:**
- **ssrf_to_cloud_metadata**: SSRF in image proxy + cloud instance metadata (Capital One pattern)
- **ssrf_to_internal_admin**: SSRF in webhook receiver + unauthenticated internal admin API

### 3. compound_injection -- One Injection Enabling Another

A first injection writes a payload that a second vulnerability renders or executes. Neither injection alone achieves the full impact.

**Scenarios:**
- **sqli_to_stored_xss**: SQL injection stores XSS payload in DB + template renders without escaping
- **deser_to_rce**: Untrusted deserialization with pickle + gadget chain to code execution

### 4. multi_stage -- Complex Multi-Hop Attacks

Multi-step attack chains requiring 2+ distinct vulnerability types in sequence.

**Scenarios:**
- **file_upload_to_rce**: Unrestricted file upload (MIME-only check) + static file serving = webshell
- **log_injection_to_xss**: User input in logs without sanitization + admin dashboard renders logs raw

---

## Safe Variant Design

The safe variant design is critical for benchmark quality. Each safe variant changes **exactly one file** with a **minimal change**:

| Scenario | File Changed | What Changes |
|----------|-------------|-------------|
| auth_bypass_to_sqli | routes.py | Auth decorator added to admin route |
| auth_bypass_to_cmdi | middleware.py | hmac.compare_digest instead of == |
| ssrf_to_cloud_metadata | gateway.py | URL host allowlist added |
| ssrf_to_internal_admin | webhook.py | Callback URL host allowlist added |
| sqli_to_stored_xss | templates.py | html.escape() applied to output |
| deser_to_rce | api.py | json.loads() instead of pickle.loads() |
| file_upload_to_rce | upload.py | File extension allowlist added |
| log_injection_to_xss | dashboard.py | html.escape() on log lines |

A tool that flags both vuln and safe variants equally is detecting individual findings, not chains. A tool that flags only the vuln variant is demonstrating chain-aware analysis.

---

## Test Case Statistics

| Category | Scenarios | Vuln | Safe | Total |
|----------|-----------|------|------|-------|
| unauth_injection | 2 | 2 | 2 | 4 |
| ssrf_pivot | 2 | 2 | 2 | 4 |
| compound_injection | 2 | 2 | 2 | 4 |
| multi_stage | 2 | 2 | 2 | 4 |
| **TOTAL** | **8** | **8** | **8** | **16** |

Exploitable/Mitigated split: 50% / 50%

---

## Known Limitations

- **Small initial size:** 8 scenarios (16 test cases). Each scenario requires multiple files with careful vuln/safe pairing. Quality over quantity for v0.1.0.
- **Python only:** All scenarios use Flask. Future versions should add Go (net/http middleware chains), JavaScript (Express middleware), and Java (Spring Security filter chains).
- **2-3 link chains only:** Real-world exploit chains can be 5+ steps. Future versions should include longer chains.
- **No cross-application chains:** All chains are within a single application. Cross-service chains (microservice A's SSRF reaches microservice B's unprotected endpoint) require multi-application scenarios.
- **Self-graded:** Same caveat as all other benchmarks. Independent verification welcome.

---

## Contributing

To add a chain scenario:

1. Create `scenarios/<name>/vuln/` and `scenarios/<name>/safe/` directories
2. Write 2-5 source files per variant with realistic Flask/Django/Express code
3. Add `vuln-code-snippet` annotations at the chain endpoint (the final sink)
4. The safe variant must change **exactly one file** with a minimal fix
5. All other files must be byte-identical between vuln and safe
6. Add CSV entries: `chain_<shortname>_vuln,<category>,true,<CWE>` and `chain_<shortname>_safe,<category>,false,<CWE>`
7. Run `python scripts/validate_chains.py` to verify L1-L5 fidelity

**Design requirements:**
- Chain must be based on a real-world attack pattern or CVE
- Individual findings must each be low/medium severity
- Compound chain must be high/critical severity
- Safe variant must break the chain by fixing one link, not by removing the endpoint
