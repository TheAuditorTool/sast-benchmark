# PHP SAST Benchmark Scoring

## Quick Start

```bash
# 1. Run your SAST tool on php/ and export SARIF 2.1.0
your-tool --output results.sarif php/

# 2. Score against ground truth (testcode/ auto-detected from CSV path)
python scripts/score_sarif.py results.sarif php/expectedresults-0.3.1.csv
```

## Detection Method: Filename-Based Matching

PHP uses filename-based matching (same as Go). Each test case is a single
file named `benchmark_test_NNNNN.php`. Files contain no annotations or
comments that reveal the vulnerability category.

A test case is **detected** if a SARIF finding references the test file
(by URI) AND the finding's ruleId CWE matches the test case's expected CWE.

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

This prevents large categories (e.g., sqli with 30 tests) from dominating small ones (e.g., weakrand with 6 tests).

**Flat Aggregate** -- For comparison:
Sum all TP/FP/FN/TN across all tests, compute global TPR and FPR.

## Ground Truth Format

```csv
# test name,category,real vulnerability,CWE
BenchmarkTest00001,weakrand,false,330
BenchmarkTest00002,ssrf,true,918
```

## Supported Tools

Any SAST tool that outputs SARIF 2.1.0 can be scored. Tool-specific instructions:

### TheAuditor
```bash
cd php/
aud full --offline
python ../scripts/convert_theauditor.py .pf/repo_index.db
python ../scripts/score_sarif.py theauditor.sarif expectedresults-0.3.1.csv
```

The converter auto-detects language and benchmark directory from the DB path,
writes `theauditor.sarif` with integrity hashes, and skips regeneration if
DB+CSV haven't changed. Use `--force` to regenerate unconditionally.

### Semgrep
```bash
semgrep --config auto --sarif -o semgrep.sarif php/
python scripts/score_sarif.py semgrep.sarif php/expectedresults-0.3.1.csv
```

### Psalm (Taint Analysis)
```bash
cd php/ && psalm --taint-analysis --output-format=sarif > psalm.sarif
python ../scripts/score_sarif.py psalm.sarif expectedresults-0.3.1.csv
```

### Bearer
```bash
bearer scan php/ --format sarif --output bearer.sarif
python scripts/score_sarif.py bearer.sarif php/expectedresults-0.3.1.csv
```

## Validation

```bash
python scripts/validate_php.py
```

Runs L1-L5 fidelity checks: structural integrity, naming convention,
schema validation, anti-target-leakage, and scoring pipeline readiness.
