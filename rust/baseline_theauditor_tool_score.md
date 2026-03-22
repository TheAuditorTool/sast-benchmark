# Baseline Score: TheAuditor v3.5 against Rust Benchmark v0.3.1

**Tool:** TheAuditor v3.5
**Date:** 2026-03-20
**Configuration:** `aud full --offline` (default settings, no tuning)
**Benchmark:** 262 test cases, 13 CWE categories, 4 frameworks

---

## Scorecard

```
Category         CWE    TP    FP    FN    TN      TPR     FPR   Score
----------------------------------------------------------------------
cmdi             78     8     6     6     10    57.1%   37.5%  +19.6%
crypto           327    0     0     9     11     0.0%    0.0%   +0.0%
deser            502    0     0     5     7      0.0%    0.0%   +0.0%
infodisclosure   200    0     0     8     8      0.0%    0.0%   +0.0%
inputval         20     0     0     4     6      0.0%    0.0%   +0.0%
intoverflow      190    0     0     5     7      0.0%    0.0%   +0.0%
memsafety        119    8     5     0     7    100.0%   41.7%  +58.3%
pathtraver       22     0     0     14    14     0.0%    0.0%   +0.0%
redos            1333   0     0     5     5      0.0%    0.0%   +0.0%
sqli             89     0     0     24    26     0.0%    0.0%   +0.0%
ssrf             918    0     0     9     13     0.0%    0.0%   +0.0%
weakrand         330    0     0     7     9      0.0%    0.0%   +0.0%
xss              79     0     0     5     11     0.0%    0.0%   +0.0%
----------------------------------------------------------------------
OVERALL                 16    11   101   134    13.7%    7.6%   +6.1%
```

Score = TPR - FPR (Youden's J). 0% = random guessing. +100% = perfect.

---

## What This Means

**+6.1% overall.** Two categories have detection. Eleven have zero.

### What works

**memsafety (+58.3%)** — 100% recall, catches every unsafe block, transmute, raw pointer, and get_unchecked pattern. But 41.7% FPR: the structural rules flag 5 safe patterns (bounds-checked unsafe, safe MaybeUninit, copy_from_slice) as dangerous. The rules detect `unsafe` keyword presence but don't distinguish safe usage from unsafe usage.

**cmdi (+19.6%)** — Detects 8 of 14 command injection patterns (57.1% recall). Catches `Command::new(user_input)` and shell execution via `sh -c`. Misses 6 patterns where command names come from match allowlists or arguments are sanitized — those are FN because the structural detector fires on `Command::new` regardless of context. 6 FP where safe patterns (hardcoded commands, stdin piping, argument filtering) are flagged.

### What doesn't work (0% — no detection)

**sqli (0/24 TP)** — Rules exist (`rust_injection_analyze.py` with Track A taint + Track B `format!` detection) but produced zero findings on benchmark code. Root cause to investigate: either taint flows don't connect across files (handler → database function), or the `format!` structural detector doesn't match the patterns in these specific files, or finding file paths don't match annotation paths.

**pathtraver (0/14 TP)** — Polyglot `path_traversal_analyze.py` covers `.rs` via Track A taint. Zero findings. Taint sources (actix extractors) may not connect to file operation sinks (`fs::read_to_string`, `fs::write`) through the graph.

**ssrf (0/9 TP)** — Same architecture as pathtraver. Polyglot Track A rule, zero findings.

**crypto, deser, infodisclosure, inputval, intoverflow, redos, weakrand, xss** — No Rust-specific rules exist for these categories. Expected 0%. These are documented engine gaps (see `coverage_cve_gaps.md` in TheAuditorV2 root).

### False Positives (11 total)

| Category | FP | What's being falsely flagged |
|----------|----|-----------------------------|
| cmdi | 6 | Safe command patterns: hardcoded commands, argument filtering, stdin piping, shell escaping |
| memsafety | 5 | Safe unsafe patterns: bounds-checked access, safe MaybeUninit, copy_from_slice |

---

## Interpretation

This is an untuned, out-of-the-box baseline. The Rust taint pipeline and rules are younger than Java/Python (which achieved 100% on their respective benchmarks). This score establishes the starting point.

Every FN is a ticket. Every FP is a precision fix. The benchmark exists to make these visible and track improvement over time.

| Metric | Value |
|--------|-------|
| True Positives | 16 of 117 (13.7% recall) |
| False Positives | 11 of 145 (7.6% fall-out) |
| False Negatives | 101 (missed vulnerabilities) |
| True Negatives | 134 (correctly ignored safe code) |
| Categories with detection | 2 of 13 |
| Categories at 0% | 11 of 13 |

---

## Version History

| Date | Tool Version | Score | TP | FP | FN | TN | Notes |
|------|-------------|-------|----|----|----|----|-------|
| 2026-03-20 | v3.5 | +6.1% | 16 | 11 | 101 | 134 | Baseline, untuned |

**Note (2026-03-22):** Ground truth updated to v0.3.1 — 12 misclassified test cases fixed (see CHANGELOG.md). Baseline score above was computed against v0.3 ground truth. Re-scoring against v0.3.1 ground truth pending.
