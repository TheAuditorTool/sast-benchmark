# Scoring the Rust SAST Benchmark

## Quick Start

```bash
# 1. Run your SAST tool and export to SARIF
your-tool scan ./apps/ ./testcode/ --output results.sarif

# 2. Score against ground truth
python ../scripts/score_sarif.py results.sarif expectedresults-0.5.0.csv \
```

The scorer outputs two tables: **Category-Averaged** (OWASP standard) and **Flat Aggregate** (for comparison).

---

## How Rust Scoring Works

Unlike Go (one file per test, filename-based matching), Rust test cases are identified by `vuln-code-snippet` annotations embedded in source files. Multiple test cases can exist in a single file (all `apps/` test cases share files with other functions).

A finding is matched to a test case when the SARIF result's `(file, line)` falls within the annotation's `(file, start_line, end_line)` range. The scorer auto-detects annotation mode when CSV keys don't follow the Go `BenchmarkTestNNNNN` pattern.

---

## Supported Tools

Any SAST tool that outputs SARIF 2.1.0 can be scored.

### Semgrep

```bash
semgrep --config auto --sarif --output results.sarif ./apps/ ./testcode/
python ../scripts/score_sarif.py results.sarif expectedresults-0.5.0.csv \
```

### CodeQL

```bash
codeql database create rust-bench-db --language=rust --source-root=.
codeql database analyze rust-bench-db codeql/rust-queries \
    --format=sarifv2.1.0 --output=results.sarif

python ../scripts/score_sarif.py results.sarif expectedresults-0.5.0.csv \
```

### TheAuditor

TheAuditor uses a proprietary database format. Use the bridge script:

```bash
# Run TheAuditor
aud full --offline

# Convert to SARIF
python ../scripts/convert_theauditor.py .pf/repo_index.db

# Score
python ../scripts/score_sarif.py theauditor.sarif expectedresults-0.5.0.csv
```

---

## Scoring Methodology

### Annotation-Based Detection

A test case is "detected" if the tool produces **at least one SARIF result** whose physical location (file path + line number) falls within the test case's `vuln-code-snippet start/end` annotation range. This allows multiple test cases per file and precise line-level matching.

### Confusion Matrix

For each test case, the scorer compares detection (yes/no) against ground truth (vulnerable/safe):

```
                    Tool says YES     Tool says NO
Ground truth TRUE:  True Positive     False Negative
Ground truth FALSE: False Positive    True Negative
```

### Category-Averaged Scoring (OWASP Standard)

For each vulnerability category:

```
TPR = TP / (TP + FN)    Sensitivity (catch rate)
FPR = FP / (FP + TN)    Fall-out (false alarm rate)
Score = TPR - FPR        Youden's J statistic
```

Overall score is the **macro average** across all 20 categories. Each category weighted equally regardless of test count.

### Score Interpretation

| Score | Meaning |
|-------|---------|
| +100% | Perfect -- catches everything, zero false alarms |
| 0% | Random guessing -- no better than flipping a coin |
| -100% | Perfectly wrong -- flags safe code, misses vulnerable code |

---

## Ground Truth Format

The expected results CSV (`expectedresults-0.5.0.csv`) has four fields per line:

```
# test name,category,real vulnerability,CWE
testcodeCmdi001,cmdi,true,78
testcodeCmdi002,cmdi,false,78
sqliSearchUsersVulnerable,sqli,true,89
```

- **test name**: Matches annotation key in source (`vuln-code-snippet start <key>`)
- **category**: Short vulnerability category name
- **real vulnerability**: `true` = actually vulnerable, `false` = safe
- **CWE**: CWE number for the vulnerability class

---

## Adding a New Tool's Baseline Score

1. Run your tool on the `rust/` directory
2. Export findings as SARIF 2.1.0
3. Score: `python scripts/score_sarif.py <output.sarif> rust/expectedresults-0.5.0.csv`
4. Submit a PR adding `baseline_<toolname>_score.md` to `rust/` with the full scorecard

Include tool name, version, date, and whether benchmark-specific tuning was applied.
