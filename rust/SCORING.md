# Scoring the Rust SAST Benchmark

## Quick Start

```bash
# 1. Run your SAST tool and export to SARIF
your-tool scan ./testcode/ --output results.sarif

# 2. Score against ground truth
python ../scripts/score_sarif.py results.sarif expectedresults-0.5.1.csv \
```

The scorer outputs two tables: **Category-Averaged** (OWASP standard) and **Flat Aggregate** (for comparison).

---

## How Rust Scoring Works

Rust uses filename-based matching. Each file is `benchmark_test_NNNNN.rs` — one file, one test case, one `pub fn handle()`. CSV key `BenchmarkTestNNNNN` maps directly to the filename. No annotations needed.

Reference apps (8 applications) moved to `vulnerable_apps/rust/` for separate scoring.

---

## Supported Tools

Any SAST tool that outputs SARIF 2.1.0 can be scored.

### Semgrep

```bash
semgrep --config auto --sarif --output results.sarif ./testcode/
python ../scripts/score_sarif.py results.sarif expectedresults-0.5.1.csv \
```

### CodeQL

```bash
codeql database create rust-bench-db --language=rust --source-root=.
codeql database analyze rust-bench-db codeql/rust-queries \
    --format=sarifv2.1.0 --output=results.sarif

python ../scripts/score_sarif.py results.sarif expectedresults-0.5.1.csv \
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

Overall score is the **macro average** across all 25 categories. Each category weighted equally regardless of test count.

### Score Interpretation

| Score | Meaning |
|-------|---------|
| +100% | Perfect -- catches everything, zero false alarms |
| 0% | Random guessing -- no better than flipping a coin |
| -100% | Perfectly wrong -- flags safe code, misses vulnerable code |

---

## Ground Truth Format

The expected results CSV (`expectedresults-0.5.1.csv`) has four fields per line:

```
# test name,category,real vulnerability,CWE
BenchmarkTest00967,sqli,true,89
BenchmarkTest00040,authnfailure,false,287
sqliSearchUsersVulnerable,sqli,true,89
```

- **test name**: For testcode, `BenchmarkTestNNNNN` matching `benchmark_test_NNNNN.rs`. For apps, matches annotation key (`vuln-code-snippet start <key>`).
- **category**: Short vulnerability category name
- **real vulnerability**: `true` = actually vulnerable, `false` = safe
- **CWE**: CWE number for the vulnerability class

---

## Adding a New Tool's Baseline Score

1. Run your tool on the `rust/` directory
2. Export findings as SARIF 2.1.0
3. Score: `python scripts/score_sarif.py <output.sarif> rust/expectedresults-0.5.1.csv`
4. Submit a PR adding `baseline_<toolname>_score.md` to `rust/` with the full scorecard

Include tool name, version, date, and whether benchmark-specific tuning was applied.
