# Baseline Score: TheAuditor (Adversarial Evasion Benchmark)

**Tool:** TheAuditor (java-support branch)
**Benchmark:** Adversarial Evasion Benchmark v0.2.0 (123 test cases, 10 categories)
**Date:** 2026-04-01
**Configuration:** `aud full --offline` (default rules, no tuning)

---

## Why We Publish This

We built this benchmark. We also built the tool being scored. Publishing our own tool's results -- including every missed evasion pattern and every false alarm -- is how we hold ourselves accountable. Every FN below is a detection gap we intend to close.

---

## Scorecard

```
Category               CWE    TP    FP    FN    TN      TPR     FPR   Score
--------------------------------------------------------------------------------
unicode_payload        506    2     0     3     5     40.0%    0.0%  +40.0%
dynamic_construction   506    2     1     4     3     33.3%   25.0%   +8.3%
ai_prompt_injection    1059   0     0     13    13     0.0%    0.0%   +0.0%
c2_fingerprint         506    0     0     5     5      0.0%    0.0%   +0.0%
charset_mapping        838    0     0     6     5      0.0%    0.0%   +0.0%
llm_code_generation    506    0     0     5     5      0.0%    0.0%   +0.0%
slopsquatting          829    0     0     5     5      0.0%    0.0%   +0.0%
steganographic_payload 506    0     1     5     4      0.0%   20.0%  -20.0%
supply_chain           506    0     0     8     8      0.0%    0.0%   +0.0%
visual_deception       451    0     0     6     4      0.0%    0.0%   +0.0%
--------------------------------------------------------------------------------
OVERALL                       4     2     60    57     6.2%    3.4%   +2.9%
```

**Category-Averaged Score: +2.8%** | **Flat Score: +2.9%** (Youden's J = TPR - FPR)

Scored via unified pipeline: `convert_theauditor.py` -> `score_sarif.py`

---

## What The Numbers Mean

- **4 True Positives**: Evasion patterns correctly detected
- **57 True Negatives**: Legitimate code correctly ignored
- **60 False Negatives**: Evasion patterns missed -- detection gaps
- **2 False Positives**: Legitimate code incorrectly flagged

---

## Strengths

| Category | Score | Why |
|----------|-------|-----|
| unicode_payload (+40.0%) | EIDL PAYLOAD_CAMOUFLAGE signal detects ZWC binary encoding and Tag Block smuggling. 2 of 5 TPs detected. Zero FP. |
| dynamic_construction (+8.3%) | `code-obfuscation-charcode` rule detects fromCharCode reconstruction and hex+rot13 chains. 2 of 6 TPs detected. 1 FP on charcode_logging (emoji icon use of fromCharCode). |

---

## Weaknesses (0% Categories -- 8 of 10)

| Category | Gap | What's Needed |
|----------|-----|---------------|
| visual_deception (0%) | No bidi override detection, no homoglyph detection, no confusables DB | Byte-level scan for bidi codepoints (U+202A-U+202E, U+2066-U+2069), Unicode confusables database for identifier comparison |
| supply_chain (0%) | No install hook analysis, no resource file payload detection | `setup.py` exec/compile detection, `postinstall` script analysis, base64-in-data-file detection |
| ai_prompt_injection (0%) | No adversarial comment/docstring detection, no config file poisoning detection | NLP-level intent analysis for comments, AGENTS.md/CLAUDE.md instruction scanning |
| c2_fingerprint (0%) | No unusual API pattern detection | Behavioral analysis: API response -> decode -> eval pattern detection |
| charset_mapping (0%) | No Windows Best-Fit mapping detection | Unicode codepoint database for chars that map to security-sensitive ASCII under CP932/CP949/CP1252 |
| steganographic_payload (0%) | No binary-file-read-to-exec pattern detection | Taint through file read -> decode -> exec for non-code file types |
| slopsquatting (0%) | No package existence verification | Import resolution against known package registries or hallucination database |
| llm_code_generation (0%) | No LLM-response-to-exec detection | Taint through API response -> eval/exec for LLM API endpoints |

---

## False Positive Analysis

| Key | Category | File | Why It's FP |
|-----|----------|------|-------------|
| charcode_logging | dynamic_construction | js_code_assembly.js:59-70 | Uses `String.fromCharCode` for log level icons (checkmark, warning, cross mark). The `code-obfuscation-charcode` rule fires on any fromCharCode usage but this is legitimate emoji/icon construction, not code assembly. Fix: add heuristic for small charcode sequences producing display characters vs large sequences producing executable strings. |

---

## Detection Gap Roadmap

**Quick wins (rule additions, no new paradigm):**
1. Bidi override detection -- scan for U+202A-U+202E in non-string contexts
2. Install hook exec detection -- flag exec/compile in setup.py and postinstall scripts
3. charcode_logging FP fix -- distinguish display chars from executable reconstruction

**Medium effort (new detection patterns):**
4. Homoglyph identifier detection -- Unicode confusables DB comparison
5. Charset mapping detection -- CP mapping database for dangerous transformations
6. Steganographic pattern -- taint through file-read-of-binary -> decode -> exec

**Hard (new paradigm required):**
7. AI prompt injection -- requires NLP-level adversarial intent detection in comments
8. C2 fingerprint -- requires behavioral modeling of API-response-to-exec patterns
9. Slopsquatting -- requires package registry integration or hallucination DB
10. LLM code generation -- requires LLM API endpoint recognition + response-to-exec taint
