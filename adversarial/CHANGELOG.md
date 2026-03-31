# Adversarial Evasion Benchmark Changelog

## v0.2.0 (2026-03-31)

- Expanded from 60 to 123 test cases across 10 categories (was 6)
- **New category: `charset_mapping`** (CWE-838) -- 11 test cases
  - Windows Best-Fit / WorstFit encoding attacks
  - Based on DEVCORE disclosure (Jan 2025), CVE-2024-4577 (CVSS 9.8)
  - Soft Hyphen -> dash, Yen -> backslash, Won -> backslash, Fullwidth Hyphen -> dash
- **New category: `steganographic_payload`** (CWE-506) -- 10 test cases
  - Payloads hidden in non-code files (PNG, EXIF, font, SVG, PDF)
  - Based on buildrunner-dev npm (Feb 2026), XWorm PNG campaign (Mar 2025)
- **New category: `slopsquatting`** (CWE-829) -- 10 test cases
  - AI-hallucinated package name imports
  - Based on UT San Antonio research (Mar 2025), Trend Micro dataset
- **New category: `llm_code_generation`** (CWE-506) -- 10 test cases
  - LLM API response fed to eval/exec/innerHTML
  - Based on Unit 42 LLM runtime assembly (Jan 2026), nullifAI (Feb 2025)
- **Expanded `supply_chain`** from 10 to 16 test cases (+6)
  - Shai-Hulud v2 patterns: Bun runtime evasion, self-hosted runner registration
  - S1ngularity/Nx patterns: preinstall environment exfiltration
- **Expanded `ai_prompt_injection`** from 10 to 26 test cases (+16)
  - Cross-language: added Go, Ruby, Bash test files
  - Config file poisoning: AGENTS.md and CLAUDE.md injection patterns
  - "3-step lie" patterns: adversarial instructions disguised as architectural decisions
  - Fake security scan approval metadata in script headers
  - Based on: MCPoison CVE-2025-54136, CurXecute CVE-2025-54135, Snyk ToxicSkills (2026)
- TP/TN split: 64/59 (52%/48%)
- Updated scoring script with new RULE_MAP/SINK_MAP entries
- Updated validator with new VALID_CATEGORIES and VALID_CWES

## v0.1.0 (2026-03-24)

- Initial benchmark creation: 60 test cases across 6 categories
- Categories: unicode_payload, visual_deception, dynamic_construction, supply_chain, ai_prompt_injection, c2_fingerprint
- Languages: JavaScript, Python, Go
- TP/TN split: 32/28 (53%/47%)
- Based on real-world campaigns: Glassworm (2026), os-info-checker-es6 (2025), Trojan Source CVE-2021-42574, CVE-2021-42694
