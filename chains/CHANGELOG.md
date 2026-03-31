# Chain Detection Benchmark Changelog

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
