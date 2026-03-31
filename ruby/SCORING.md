# Ruby SAST Benchmark Scoring

## Quick Start

```bash
# 1. Run your SAST tool on ruby/ and export SARIF 2.1.0
your-tool --output results.sarif ruby/

# 2. Score against ground truth (apps/ and testcode/ auto-detected from CSV path)
python scripts/score_sarif.py results.sarif ruby/expectedresults-0.1.0.csv
```

## Detection Method: Annotation-Based Matching

Ruby uses annotation-based matching (same as PHP, Rust, and Bash). Test cases are marked with `vuln-code-snippet` comments in Ruby source files:

```ruby
# vuln-code-snippet start ruby_sqli_ar_concat
def get_user(params)
  id = params[:id]
  User.where("id = #{id}") # vuln-code-snippet vuln-line ruby_sqli_ar_concat
end
# vuln-code-snippet end ruby_sqli_ar_concat
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

This prevents large categories (e.g., sqli with many tests) from dominating small ones (e.g., regex with fewer tests).

**Flat Aggregate** -- For comparison:
Sum all TP/FP/FN/TN across all tests, compute global TPR and FPR.

## Ground Truth Format

```csv
# test name,category,real vulnerability,CWE
ruby_sqli_ar_concat,sqli,true,89
ruby_sqli_ar_prepared,sqli,false,89
```

## Supported Tools

Any SAST tool that outputs SARIF 2.1.0 can be scored. Tool-specific instructions:

### TheAuditor
```bash
cd ruby/
aud full --offline
python ../scripts/convert_theauditor.py .pf/repo_index.db
python ../scripts/score_sarif.py theauditor.sarif expectedresults-0.1.0.csv
```

The converter auto-detects language and benchmark directory from the DB path,
writes `theauditor.sarif` with integrity hashes, and skips regeneration if
DB+CSV haven't changed. Use `--force` to regenerate unconditionally.

### Brakeman
```bash
brakeman -o brakeman.sarif --format sarif ruby/
python scripts/score_sarif.py brakeman.sarif ruby/expectedresults-0.1.0.csv
```

### Semgrep
```bash
semgrep --config auto --sarif -o semgrep.sarif ruby/
python scripts/score_sarif.py semgrep.sarif ruby/expectedresults-0.1.0.csv
```

## Validation

```bash
python scripts/validate_ruby.py
```

Runs L1-L5 fidelity checks: structural integrity, roundtrip fidelity, schema validation, semantic fidelity, and scoring pipeline readiness.
