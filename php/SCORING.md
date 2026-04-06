# PHP SAST Benchmark Scoring

## Quick Start

```bash
# 1. Run your SAST tool on php/ and export SARIF 2.1.0
your-tool --output results.sarif php/

# 2. Score against ground truth (apps/ and testcode/ auto-detected from CSV path)
python scripts/score_sarif.py results.sarif php/expectedresults-0.2.0.csv
```

## Detection Method: Annotation-Based Matching

PHP uses annotation-based matching (same as Rust and Bash). Test cases are marked with `vuln-code-snippet` comments in PHP source files:

```php
// vuln-code-snippet start php_sqli_pdo_concat
function getUser(PDO $pdo, $id) {
    $query = "SELECT * FROM users WHERE id = " . $id; // vuln-code-snippet vuln-line php_sqli_pdo_concat
    return $pdo->query($query);
}
// vuln-code-snippet end php_sqli_pdo_concat
```

A test case is **detected** if a SARIF finding's `(file, line)` falls within the annotation's `[start_line, end_line]` range AND the finding's ruleId category matches the test case's expected category.

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
php_sqli_pdo_concat,sqli,true,89
php_sqli_pdo_prepare,sqli,false,89
```

## Supported Tools

Any SAST tool that outputs SARIF 2.1.0 can be scored. Tool-specific instructions:

### TheAuditor
```bash
cd php/
aud full --offline
python ../scripts/convert_theauditor.py .pf/repo_index.db
python ../scripts/score_sarif.py theauditor.sarif expectedresults-0.2.0.csv
```

The converter auto-detects language and benchmark directory from the DB path,
writes `theauditor.sarif` with integrity hashes, and skips regeneration if
DB+CSV haven't changed. Use `--force` to regenerate unconditionally.

### Semgrep
```bash
semgrep --config auto --sarif -o semgrep.sarif php/
python scripts/score_sarif.py semgrep.sarif php/expectedresults-0.2.0.csv
```

### Psalm (Taint Analysis)
```bash
cd php/ && psalm --taint-analysis --output-format=sarif > psalm.sarif
python ../scripts/score_sarif.py psalm.sarif expectedresults-0.2.0.csv
```

### Bearer
```bash
bearer scan php/ --format sarif --output bearer.sarif
python scripts/score_sarif.py bearer.sarif php/expectedresults-0.2.0.csv
```

## Validation

```bash
python scripts/validate_php.py
```

Runs L1-L5 fidelity checks: structural integrity, roundtrip fidelity, schema validation, semantic fidelity, and scoring pipeline readiness.
