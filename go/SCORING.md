# Scoring the Go SAST Benchmark

## Quick Start

```bash
# 1. Run your SAST tool and export to SARIF
your-tool scan ./testcode/ --output results.sarif

# 2. Score against ground truth
python ../scripts/score_sarif.py results.sarif expectedresults-0.5.1.csv
```

The scorer outputs two tables: **Category-Averaged** (OWASP standard) and **Flat Aggregate** (for comparison).

---

## Supported Tools

Any SAST tool that outputs SARIF 2.1.0 can be scored. Below are export commands for common tools.

### Semgrep

```bash
semgrep --config auto --sarif --output results.sarif ./testcode/
python ../scripts/score_sarif.py results.sarif expectedresults-0.5.1.csv
```

### Gosec

```bash
gosec -fmt sarif -out results.sarif ./testcode/...
python ../scripts/score_sarif.py results.sarif expectedresults-0.5.1.csv
```

### CodeQL

```bash
# Create database
codeql database create go-bench-db --language=go --source-root=.

# Run queries and export SARIF
codeql database analyze go-bench-db codeql/go-queries \
    --format=sarifv2.1.0 --output=results.sarif

python ../scripts/score_sarif.py results.sarif expectedresults-0.5.1.csv
```

### Staticcheck

Staticcheck does not natively produce SARIF. Use a wrapper or convert its output:

```bash
staticcheck -f json ./testcode/... > staticcheck.json
# Convert to SARIF manually or use a community converter
```

### SonarQube

Export findings as SARIF from the SonarQube UI or API, then score:

```bash
python ../scripts/score_sarif.py sonar-results.sarif expectedresults-0.5.1.csv
```

### TheAuditor

TheAuditor uses a proprietary database format. Use the bridge script:

```bash
# Run TheAuditor
aud full --offline

# Convert to SARIF
python ../scripts/convert_theauditor.py .pf/repo_index.db

# Score
python ../scripts/score_sarif.py theauditor.sarif expectedresults-0.5.1.csv
# Converter auto-detects language, writes theauditor.sarif with integrity hashes
```

---

## Scoring Methodology

### How Detection Works

A test case is "detected" if the tool produces **at least one SARIF result** whose location URI contains the test file name (e.g., `benchmark_test_00042.go`). The scorer does NOT match on rule IDs or CWE numbers -- it uses filename-based detection only. This makes it compatible with any tool regardless of rule naming conventions.

### Confusion Matrix

For each test case, the scorer compares detection (yes/no) against ground truth (vulnerable/safe):

```
                    Tool says YES     Tool says NO
Ground truth TRUE:  True Positive     False Negative
Ground truth FALSE: False Positive    True Negative
```

### Category-Averaged Scoring (OWASP Standard)

This is the official scoring method used by OWASP BenchmarkJava and BenchmarkPython.

For each vulnerability category:

```
TPR = TP / (TP + FN)    Sensitivity (catch rate)
FPR = FP / (FP + TN)    Fall-out (false alarm rate)
Score = TPR - FPR        Youden's J statistic
```

The overall score is the **macro average** across all categories:

```
Overall TPR = mean(TPR_c for all categories c)
Overall FPR = mean(FPR_c for all categories c)
Overall Score = Overall TPR - Overall FPR
```

This treats each category equally regardless of test count. A tool that scores 90% on SQL injection (123 tests) but 0% on all other categories will get a low overall score, not a misleadingly high one.

### Flat Aggregate Scoring (for comparison)

Sum all TP/FP/FN/TN across all tests, then compute global TPR and FPR:

```
TPR = total_TP / (total_TP + total_FN)
FPR = total_FP / (total_FP + total_TN)
Score = TPR - FPR
```

This method gives more weight to larger categories (SQL injection dominates). It is provided for historical comparison but is NOT the OWASP standard.

### Score Interpretation

| Score | Meaning |
|-------|---------|
| +100% | Perfect -- catches everything, zero false alarms |
| 0% | Random guessing -- no better than flipping a coin |
| -100% | Perfectly wrong -- flags safe code, misses vulnerable code |

A tool that blindly flags everything gets TPR=100% but FPR=100%, scoring exactly 0%. A tool that flags nothing also scores 0%. Only tools that discriminate between vulnerable and safe code score above 0%.

---

## Ground Truth Format

The expected results CSV has four fields per line:

```
# test name, category, real vulnerability, CWE
BenchmarkTest00001,sqli,true,89
BenchmarkTest00002,sqli,false,89
```

- **test name**: Maps to file `testcode/benchmark_test_NNNNN.go`
- **category**: Short vulnerability category name
- **real vulnerability**: `true` = actually vulnerable, `false` = safe (should not be flagged)
- **CWE**: CWE number for the vulnerability class

---

## Adding a New Tool's Baseline Score

To contribute your tool's score to the benchmark:

1. Run your tool on the `go/` directory
2. Export findings as SARIF 2.1.0
3. Run `python scripts/score_sarif.py <your_output.sarif> go/expectedresults-0.5.1.csv`
4. Submit a PR adding a `baseline_<toolname>_score.md` to `go/` with your full scorecard

Include the tool name, version, date, and whether any benchmark-specific tuning was applied.

---

## BenchmarkUtils Compatibility

The OWASP BenchmarkJava and BenchmarkPython projects use the BenchmarkUtils Maven plugin for scoring. Our CSV format is identical to theirs (`test name, category, real vulnerability, CWE`). Future versions may integrate directly with BenchmarkUtils for cross-benchmark scorecard generation.
