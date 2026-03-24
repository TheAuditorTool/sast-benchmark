# Adversarial Evasion Benchmark

**Created:** 2026-03-24 | **Version:** v0.1.0
**Test Cases:** 60 (32 TP / 28 TN) across 6 categories
**Languages:** JavaScript, Python, Go (cross-language)

---

## Purpose

This benchmark tests a fundamentally different capability than traditional SAST benchmarks. While the Go/Rust/Bash/PHP benchmarks ask "can your tool find this vulnerability?", this benchmark asks: **"can your tool detect that someone is hiding something?"**

Traditional SAST is blind to these attack classes because the AST, regex, and taint analysis see "normal" code. Detection requires new paradigms: byte-level Unicode scanning, visual deception analysis, behavioral intent modeling, and supply chain anomaly detection.

**Real-world motivation:**
- Glassworm campaign (March 2026): 433 repos compromised via invisible Variation Selector encoding
- os-info-checker-es6 (May 2025): Variation Selector steganography + Google Calendar C2
- Trojan Source (CVE-2021-42574): Bidi overrides making code display differently than it executes
- Slopsquatting & AI prompt injection: Attacks targeting AI coding assistants

---

## Categories

### 1. unicode_payload (CWE-506)
Invisible Unicode characters hiding payloads in string literals. Variation Selectors, zero-width characters, tag block characters, BOM padding.

### 2. visual_deception (CWE-451)
Code that displays differently than it executes. Bidi overrides (Trojan Source), homoglyph identifier substitution (Cyrillic/Latin confusables), mixed-script attacks.

### 3. dynamic_construction (CWE-506)
Building dangerous operations from fragments at runtime. String.fromCharCode, bytes.fromhex, getattr with concatenation, multi-layer encoding chains (base64+hex+rot13).

### 4. supply_chain (CWE-506)
Malicious code in package install hooks and non-code resource files. setup.py exec, postinstall curl|node, base64 payloads in data files, code disguised as test fixtures.

### 5. ai_prompt_injection (CWE-1059)
Hidden instructions targeting AI coding assistants and AI-based security scanners. Malicious comments, docstring manipulation, poisoned config files (.cursorrules), invisible tag-block prompts, anti-AI "this file is safe" claims.

### 6. c2_fingerprint (CWE-506)
Non-standard command-and-control communication channels. Solana blockchain transaction memos, Google Calendar event descriptions, DNS TXT records, Ethereum smart contract storage, pastebin raw pastes.

---

## Detection Requirements

| Category | AST Analysis | Regex/Pattern | Byte-Level Scan | Behavioral/Intent |
|---|---|---|---|---|
| unicode_payload | No | No | **Yes** | Yes (EIDL PAYLOAD_CAMOUFLAGE) |
| visual_deception | Partial | Yes (bidi ranges) | **Yes** | Yes (confusables DB) |
| dynamic_construction | No | Partial | No | **Yes** (taint through decode chains) |
| supply_chain | Yes (eval in hooks) | Yes | No | **Yes** (file context analysis) |
| ai_prompt_injection | No (comments stripped) | Partial | **Yes** (tag blocks) | **Yes** (NLP-level) |
| c2_fingerprint | Partial | Partial | No | **Yes** (unusual API patterns) |

---

## Scoring

Same formula as all other benchmarks in this suite: Youden's J statistic.

```
Score = TPR - FPR
TPR = TP / (TP + FN)
FPR = FP / (FP + TN)
```

Run: `python adversarial_benchmark.py`

---

## Test Case Statistics

| Category | CWE | Total | TP | TN |
|---|---|---|---|---|
| unicode_payload | 506 | 10 | 5 | 5 |
| visual_deception | 451 | 10 | 6 | 4 |
| dynamic_construction | 506 | 10 | 6 | 4 |
| supply_chain | 506 | 10 | 5 | 5 |
| ai_prompt_injection | 1059 | 10 | 5 | 5 |
| c2_fingerprint | 506 | 10 | 5 | 5 |
| **TOTAL** | | **60** | **32** | **28** |

TP/TN split: 53% / 47%

---

## Adversarial Design Philosophy

1. **Ground truth written BEFORE tool rules** -- prevents unconscious tailoring
2. **Every TP has TN siblings** -- discrimination is the test, not detection alone
3. **Real attack patterns** -- based on documented campaigns, not synthetic toys
4. **Cross-language** -- same attack class tested across JS/Python/Go
5. **Intent over content** -- tests whether tools detect hiding behavior, not payload signatures
6. **Future-proof** -- detecting invisible Unicode catches ALL encoding schemes, not just Glassworm

---

## Known Limitations

- **Antivirus interference:** Some test files contain patterns that trigger AV heuristics. This is by design -- the adversarial content must be realistic.
- **Self-exam bias:** We wrote the benchmark and the detection tool. Same caveat as existing benchmarks.
- **Small initial size:** 60 test cases vs 356+ in mature benchmarks. Will grow with real-world attack evolution.
- **Not covered:** Slopsquatting (ecosystem-level), dependency confusion (registry-level), AI polymorphic malware (runtime), best-fit charset mapping (requires OS-level modeling).
