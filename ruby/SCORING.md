# Ruby SAST Benchmark Scoring

## Quick Start

```bash
# 1. Run your SAST tool on ruby/ and export SARIF 2.1.0
your-tool --output results.sarif ruby/

# 2. Score against ground truth (testcode/ auto-detected from CSV path)
python scripts/score_sarif.py results.sarif ruby/expectedresults-0.3.2.csv
```

## Detection Method: Filename-Based Matching

Ruby uses filename-based matching with CWE awareness (same as Go and OWASP Java).
Each test file is `benchmark_test_NNNNN.rb` with CSV key `BenchmarkTestNNNNN`.

A test case is **detected** if a SARIF finding references the test file AND the
finding's ruleId CWE matches the test case's expected CWE.

## Scoring Formula

```
TPR = TP / (TP + FN)    Sensitivity (catch rate)
FPR = FP / (FP + TN)    Fall-out (false alarm rate)

Score = TPR - FPR        Youden's J statistic
```

| Score | Meaning |
|-------|---------|
| +100% | Perfect -- catches everything, zero false alarms |
| 0% | Random guessing -- no better than flipping a coin |
| -100% | Perfectly wrong -- flags safe code, misses vulnerable code |

## Two Scoring Methods

**Category-Averaged (OWASP Standard)** -- Primary metric:
1. Compute TPR and FPR independently for each category
2. Average all category TPRs for overall TPR
3. Average all category FPRs for overall FPR
4. Score = averaged_TPR - averaged_FPR

This prevents large categories (e.g., sqli with many tests) from dominating small ones (e.g., regex with fewer tests).

**Flat Aggregate** -- For comparison:
Sum all TP/FP/FN/TN across all tests, compute global TPR and FPR.

## Ground Truth Format

```csv
# test name,category,real vulnerability,CWE
BenchmarkTest00001,ssrf,true,918
BenchmarkTest00002,weakcipher,true,327
```

## Supported Tools

Any SAST tool that outputs SARIF 2.1.0 can be scored. Tool-specific instructions:

### TheAuditor
```bash
cd ruby/
aud full --offline
python ../scripts/convert_theauditor.py .pf/repo_index.db
python ../scripts/score_sarif.py theauditor.sarif expectedresults-0.3.2.csv
```

The converter auto-detects language and benchmark directory from the DB path,
writes `theauditor.sarif` with integrity hashes, and skips regeneration if
DB+CSV haven't changed. Use `--force` to regenerate unconditionally.

### Brakeman
```bash
brakeman -o brakeman.sarif --format sarif ruby/
python scripts/score_sarif.py brakeman.sarif ruby/expectedresults-0.3.2.csv
```

### Semgrep
```bash
semgrep --config auto --sarif -o semgrep.sarif ruby/
python scripts/score_sarif.py semgrep.sarif ruby/expectedresults-0.3.2.csv
```

## Validation

```bash
python scripts/validate_ruby.py
```

Runs L1-L6 fidelity checks: structural integrity, roundtrip fidelity, schema validation, semantic fidelity, scoring pipeline readiness, and SARIF integrity.

### CWE-94 Disambiguation (codeinj vs dynmethod)

Both `codeinj` and `dynmethod` categories use CWE-94. With filename-based matching,
a SARIF finding with CWE-94 on a file is matched to the CSV entry for that file.
Since each file maps to exactly one category, the CWE collision does not affect
scoring accuracy.
