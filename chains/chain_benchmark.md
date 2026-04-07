# Chain Detection Benchmark

**Created:** 2026-03-31 | **Updated:** 2026-04-08 | **Version:** v0.2.1
**Test Cases:** 500 (250 exploitable chains / 250 mitigated chains) across 20 categories
**Scenarios:** 250 multi-file applications in Python (Flask)

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
scenarios/scenario_NNNN/
  variant_a/
    app.py
    middleware.py
    routes.py
  variant_b/
    app.py
    middleware.py
    routes.py
```

One variant has an exploitable compound vulnerability. The other breaks exactly one link in the chain, making the compound path non-exploitable. Which variant is exploitable is randomized and recorded only in the CSV. All other files are byte-identical between variants.

### What Counts as Chain Detection

A finding counts as a chain detection ONLY if it maps to a **chain-specific category** (one of the 20 categories below). Standard individual SAST findings (sqli, cmdi, xss, etc.) do NOT count.

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

Run your SAST tool, export SARIF 2.1.0, then score:

```bash
your-tool scan scenarios/ --format sarif -o results.sarif
python ../scripts/score_sarif.py results.sarif expectedresults-0.2.0.csv
```

Any tool that produces standard SARIF can be scored.

---

## Categories

### 1. unauth_injection -- Authentication Bypass Enabling Injection

Missing or bypassable authentication that exposes injection vulnerabilities to unauthenticated attackers. The injection alone is medium (requires auth). The auth gap alone is medium. Together: critical unauthenticated injection.

**Real-world basis:** CVE-2023-22515 (Confluence auth bypass to RCE), OWASP A01+A03.

### 2. ssrf_pivot -- SSRF Used to Reach Internal Services

Server-side request forgery in a public-facing endpoint used to reach internal services that trust the network boundary. The SSRF alone is medium (hits external sites). The internal service alone is accepted risk (behind firewall). Together: critical internal service compromise.

**Real-world basis:** Capital One 2019 (SSRF + EC2 metadata), CVE-2021-26855 (Exchange SSRF).

### 3. compound_injection -- One Injection Enabling Another

A first injection writes a payload that a second vulnerability renders or executes. Neither injection alone achieves the full impact.

**Real-world basis:** CVE-2019-15107 (Webmin second-order injection), stored XSS via SQLi.

### 4. multi_stage -- Complex Multi-Hop Attacks

Multi-step attack chains requiring 2+ distinct vulnerability types in sequence.

**Real-world basis:** CVE-2024-23897 (Jenkins file read + RCE), CVE-2015-7501 (JBoss deser chain).

### 5. privesc_chain -- Privilege Escalation via Chained Weaknesses

Low-privilege access combined with missing authorization on admin functions enables privilege escalation.

**Real-world basis:** OWASP A01 (Broken Access Control), widespread RBAC bypass patterns.

### 6. idor_data_leak -- IDOR Enabling Mass Data Exfiltration

Broken object-level authorization combined with a sensitive data endpoint enables mass data leakage.

**Real-world basis:** Optus 2022 (sequential ID enumeration), Facebook phone number scraping 2021.

### 7. race_condition_bypass -- Race Condition Bypassing Security Controls

TOCTOU (time-of-check-to-time-of-use) race conditions that bypass check-then-act security controls.

**Real-world basis:** CVE-2016-5195 (Dirty COW), CVE-2022-0847 (Dirty Pipe), double-spend in fintech.

### 8. path_traversal_to_read -- Path Traversal Enabling Sensitive File Read

Path traversal on a file-serving endpoint combined with accessible sensitive files (configs, secrets, credentials).

**Real-world basis:** CVE-2021-41773 (Apache 2.4.49 path traversal), CVE-2024-23897 (Jenkins file read).

### 9. open_redirect_to_phish -- Open Redirect Enabling Credential Theft

Unvalidated redirect on a trusted domain combined with OAuth/login flows enables token or credential theft.

**Real-world basis:** OAuth redirect_uri attacks, SSO relay state manipulation.

### 10. xxe_to_file_read -- XXE Enabling Server-Side File Read

XML parser with external entity resolution enabled combined with file:// URI access to server files.

**Real-world basis:** CVE-2014-3529 (Apache POI XXE), CVE-2018-11776 (Struts XXE to RCE).

### 11. csrf_to_state_change -- CSRF Enabling Unauthorized State Changes

Missing CSRF protection combined with state-changing endpoints enables unauthorized actions via victim's session.

**Real-world basis:** WordPress 5.1 CSRF-to-RCE (2019), TikTok CSRF account takeover (2020).

### 12. header_injection_to_cache_poison -- Header Injection Enabling Cache Poisoning

HTTP response header injection combined with caching enables persistent XSS via poisoned cache entries.

**Real-world basis:** PortSwigger practical cache poisoning research (2018-2023).

### 13. weak_crypto_to_forge -- Weak Cryptography Enabling Token Forgery

Weak or predictable cryptographic implementations combined with token verification enables authentication bypass via forged tokens.

**Real-world basis:** Sony PS3 ECDSA key recovery (2010), JWT weak secret attacks.

### 14. mass_assign_to_privesc -- Mass Assignment Enabling Privilege Escalation

Unfiltered request body binding combined with role/admin fields on the model enables users to self-promote to admin.

**Real-world basis:** Egor Homakov GitHub mass assignment (2012), Rails strong_params origin.

### 15. info_leak_to_account_takeover -- Information Disclosure Enabling Account Takeover

Debug endpoints or verbose error messages that leak secrets (tokens, API keys, passwords) combined with authentication that uses those secrets.

**Real-world basis:** Uber 2016 (leaked AWS keys), CVE-2017-9841 (PHPUnit info leak to RCE).

### 16. template_injection_to_rce -- Server-Side Template Injection to RCE

User input rendered in a server-side template engine combined with the engine's code execution capabilities.

**Real-world basis:** CVE-2019-11581 (Jira SSTI to RCE), CVE-2023-46747 (F5 BIG-IP SSTI).

### 17. hardcoded_creds_to_access -- Hardcoded Credentials Enabling Unauthorized Access

Credentials hardcoded in source code combined with authentication endpoints that accept those credentials.

**Real-world basis:** CVE-2022-26138 (Confluence hardcoded password), SolarWinds (hardcoded FTP creds).

### 18. insecure_file_perms_to_tamper -- Insecure File Permissions Enabling Data Tampering

World-writable configuration or data files combined with application logic that trusts file contents.

**Real-world basis:** CVE-2016-1247 (nginx log permission escalation), Docker socket exposure attacks.

### 19. cors_miscfg_to_data_theft -- CORS Misconfiguration Enabling Cross-Origin Data Theft

Overly permissive CORS policy (reflecting origin or wildcard with credentials) combined with sensitive API endpoints.

**Real-world basis:** Widespread CORS misconfigurations documented on HackerOne.

### 20. session_fixation_to_hijack -- Session Fixation Enabling Session Hijacking

Session ID not regenerated after authentication combined with predictable or injectable session identifiers.

**Real-world basis:** CWE-384 documented attack patterns, OWASP session management guidelines.

---

## Safe Variant Design

The safe variant design is critical for benchmark quality. Each safe variant changes **exactly one file** with a **minimal fix** that breaks one link in the chain. A tool that flags both vuln and safe variants equally is detecting individual findings, not chains.

---

## Test Case Statistics

| Category | Scenarios | Vuln | Safe | Total |
|----------|-----------|------|------|-------|
| unauth_injection | 13 | 13 | 13 | 26 |
| ssrf_pivot | 13 | 13 | 13 | 26 |
| compound_injection | 13 | 13 | 13 | 26 |
| multi_stage | 13 | 13 | 13 | 26 |
| privesc_chain | 13 | 13 | 13 | 26 |
| xxe_to_file_read | 13 | 13 | 13 | 26 |
| open_redirect_to_phish | 13 | 13 | 13 | 26 |
| weak_crypto_to_forge | 13 | 13 | 13 | 26 |
| info_leak_to_account_takeover | 13 | 13 | 13 | 26 |
| template_injection_to_rce | 13 | 13 | 13 | 26 |
| idor_data_leak | 12 | 12 | 12 | 24 |
| race_condition_bypass | 12 | 12 | 12 | 24 |
| path_traversal_to_read | 12 | 12 | 12 | 24 |
| csrf_to_state_change | 12 | 12 | 12 | 24 |
| header_injection_to_cache_poison | 12 | 12 | 12 | 24 |
| mass_assign_to_privesc | 12 | 12 | 12 | 24 |
| hardcoded_creds_to_access | 12 | 12 | 12 | 24 |
| insecure_file_perms_to_tamper | 12 | 12 | 12 | 24 |
| cors_miscfg_to_data_theft | 12 | 12 | 12 | 24 |
| session_fixation_to_hijack | 12 | 12 | 12 | 24 |
| **TOTAL** | **250** | **250** | **250** | **500** |

Exploitable/Mitigated split: 50% / 50%

---

## Anti-Target Leakage Rules

Test files must not reveal the vulnerability type or expected result to the scanner. These rules are enforced by `validate_chains.py` L4.

- **Opaque directory naming:** Scenario directories use `scenario_NNNN/variant_a|b/` naming. Directory names must not contain vulnerability categories, CWE numbers, or the words "vuln"/"safe".
- **Zero comments:** Test files must contain no `#` comments except `vuln-code-snippet` annotations with opaque keys. No docstrings (module, function, or class level).
- **Opaque annotation keys:** Annotation keys use `ChainScenarioNNNNX` format (scenario number + variant letter). Keys must not encode the vulnerability type or exploitability.
- **Unified target-line:** Both exploitable and mitigated variants use `target-line` (not `vuln-line`/`safe-line`). The CSV is the sole source of truth for exploitability.
- **1 scenario = 1 test:** Each scenario directory is one test case with exactly two variants. File names within variants must be domain-descriptive (e.g., `app.py`, `routes.py`), not category-descriptive.
- **Randomized variant assignment:** Which variant is `a` vs `b` is randomized per scenario. There is no correlation between variant letter and exploitability.

---

## Known Limitations

- **Python only:** All scenarios use Flask. Future versions should add Go (net/http middleware chains), JavaScript (Express middleware), and Java (Spring Security filter chains).
- **2-3 link chains only:** Real-world exploit chains can be 5+ steps. Future versions should include longer chains.
- **No cross-application chains:** All chains are within a single application. Cross-service chains (microservice A's SSRF reaches microservice B's unprotected endpoint) require multi-application scenarios.
- **Self-graded:** Same caveat as all other benchmarks. Independent verification welcome.

---

## Contributing

To add a chain scenario:

1. Create `scenarios/scenario_NNNN/variant_a/` and `scenarios/scenario_NNNN/variant_b/` directories (use next available number)
2. Write 2-5 source files per variant with realistic Flask code
3. Add `vuln-code-snippet` annotations at the chain endpoint using opaque keys: `ChainScenarioNNNNA` / `ChainScenarioNNNNB`
4. Use `target-line` for both variants (not `vuln-line`/`safe-line`)
5. The mitigated variant must change **exactly one file** with a minimal fix
6. All other files must be byte-identical between variants
7. Add CSV entries: `ChainScenarioNNNNA,<category>,true|false,<CWE>` (randomly assign which letter is exploitable)
8. **No comments or docstrings** in test files -- only `vuln-code-snippet` annotations permitted
9. Run `python scripts/validate_chains.py` to verify L1-L5 fidelity

**Design requirements:**
- Chain must be based on a real-world attack pattern or CVE
- Individual findings must each be low/medium severity
- Compound chain must be high/critical severity
- Mitigated variant must break the chain by fixing one link, not by removing the endpoint
- File names must be domain-descriptive (`app.py`, `auth.py`), never category-descriptive
