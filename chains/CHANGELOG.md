# Chain Detection Benchmark Changelog

## v0.2.2 (2026-04-08)

- Flatten paired variant structure into 500 independent scenario directories
- Eliminates diff-based cheating: previously variant_a/variant_b sat side-by-side
  and differed by exactly one file, letting any tool find the answer by diffing
- All 500 scenarios are now top-level: scenario_0001/ through scenario_0500/
- No subdirectories, no variant pairing visible on disk
- Exploitable and mitigated scenarios shuffled together (seed=20260409)
- CSV keys simplified: ChainScenarioNNNN (no A/B suffix)
- Same approach as Go/Rust/Bash/PHP/Ruby single-file benchmarks: each test
  case is fully independent, CSV is sole source of truth
- validate_chains.py v3.0 updated for flat structure

## v0.2.1 (2026-04-08)

- Anti-target-leakage migration: eliminate all information leakage from test files
- Scenario directories renamed from descriptive (`auth_bypass_to_sqli/vuln/`) to opaque (`scenario_NNNN/variant_a|b/`) via seeded shuffle (seed=20260408)
- Randomized variant assignment: which letter (A/B) is exploitable is randomized per scenario
- Stripped ALL module, function, and class docstrings (1,522 files, 100% had leaky docstrings)
- Stripped ALL `#` comments except `vuln-code-snippet` annotations (~305 leaky comments removed)
- Annotation keys changed from descriptive (`chain_auth_sqli_vuln`) to opaque (`ChainScenario0142A`)
- Annotation markers unified: `vuln-line`/`safe-line` replaced with `target-line` for both variants
- CSV category-grouping comment headers removed (leaked chain types by grouping)
- validate_chains.py v2.0: L4 updated for target-line and anti-leakage checks
- No test case content changes: all 500 test cases, 20 categories, 250/250 balance preserved
- rename_map.json written for full auditability (old name -> new name, old key -> new key)
- Migration script: scripts/migrate_chains_leakage.py (LibCST-based, 5 validation gates)

## v0.2.0 (2026-04-07)

- Expanded from 8 to 250 scenarios (16 to 500 test cases)
- 16 new categories added (4 to 20 total):
  privesc_chain, idor_data_leak, race_condition_bypass,
  path_traversal_to_read, open_redirect_to_phish, xxe_to_file_read,
  csrf_to_state_change, header_injection_to_cache_poison,
  weak_crypto_to_forge, mass_assign_to_privesc,
  info_leak_to_account_takeover, template_injection_to_rce,
  hardcoded_creds_to_access, insecure_file_perms_to_tamper,
  cors_miscfg_to_data_theft, session_fixation_to_hijack
- 4 existing categories expanded from 2 to 13 scenarios each
- 4 new CWEs: 113 (Header Injection), 384 (Session Fixation),
  915 (Mass Assignment), 942 (CORS Misconfiguration)
- Exploitable/Mitigated split: 250/250 (50%/50%)
- All scenarios use multi-file Flask applications with vuln/safe pairs
- Safe variants differ by exactly one file from vuln variants
- Real-world basis for every category: CVE-2023-22515, Capital One 2019,
  CVE-2021-41773, CVE-2014-3529, CVE-2019-11581, CVE-2022-26138,
  CVE-2016-1247, Egor Homakov GitHub 2012, WordPress 5.1 CSRF-to-RCE,
  Sony PS3 ECDSA, PortSwigger cache poisoning research
- Scored via same L1-L5 fidelity validation system

## v0.1.0 (2026-03-31)

- Initial benchmark creation: 8 scenarios, 16 test cases across 4 categories
- Categories: unauth_injection, ssrf_pivot, compound_injection, multi_stage
- Language: Python (Flask)
- Exploitable/Mitigated split: 8/8 (50%/50%)
- Each scenario: multi-file Flask application with vuln/safe variants
- Safe variants differ by exactly one file from vuln variants
- Based on real-world patterns: Capital One breach 2019 (SSRF+metadata), OWASP Top 10 A01+A03 (auth bypass+injection), second-order injection (SQLi+stored XSS), CWE-502 (unsafe deserialization), CWE-434 (unrestricted upload)
- Scoring: Youden's J via chain_benchmark.py (3 detection tracks)
- Validation: L1-L5 fidelity via scripts/validate_chains.py
