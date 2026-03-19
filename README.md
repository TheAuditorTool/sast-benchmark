# OWASP-Style SAST Benchmark for Go, Rust, and Bash

The first public Static Application Security Testing (SAST) benchmark suite for **Go**, **Rust**, and **Bash** -- three languages with zero existing public SAST benchmarks.

## Why This Exists

SAST tools need ground truth to measure accuracy. Without benchmarks, you can't answer:

- **Does the tool actually detect this vulnerability?** (True Positive Rate)
- **Does the tool cry wolf on safe code?** (False Positive Rate)
- **Is the tool getting better or worse over time?** (Regression detection)

The [OWASP Benchmark for Java](https://owasp.org/www-project-benchmark/) (2,740 test cases) and [OWASP Benchmark for Python](https://github.com/owasp/benchmark-python) (1,230 test cases) proved this model works. No equivalent exists for Go, Rust, or Bash. This project fills that gap.

## Scoring Methodology

Each test case is classified as either **vulnerable** (true) or **safe** (false) in a CSV answer key. After running a SAST tool, scoring computes:

```
TP = vulnerable AND detected       (True Positive)
FN = vulnerable AND missed         (False Negative)
FP = safe AND falsely detected     (False Positive)
TN = safe AND correctly ignored    (True Negative)

TPR = TP / (TP + FN)               Sensitivity (catch rate)
FPR = FP / (FP + TN)               Fall-out (false alarm rate)

Score = TPR - FPR                  Youden's J statistic
```

| Score | Meaning |
|-------|---------|
| +100% | Perfect -- catches everything, zero false alarms |
| 0% | Random guessing -- no better than flipping a coin |
| -100% | Perfectly wrong -- flags safe code, misses vulnerable code |

## Directory Structure

```
gorustbash_benchmark/
  README.md                  # This file
  LICENSE                    # Apache 2.0
  CONTRIBUTING.md            # How to contribute test cases
  coverage_cve_gaps.md       # Cross-language SAST coverage gap analysis
  go/                        # Go benchmark
    go_benchmark.md
    go_ground_truth.yml
    apps/
  rust/                      # Rust benchmark (98 test cases, 13 CWEs)
    rust_benchmark.md        # Scoring script, methodology, scorecard
    rust_ground_truth.yml    # Answer key (98 test cases)
    CHANGELOG.md             # Every change documented
    apps/                    # 8 Rust applications with vuln-code-snippet annotations
  bash/                      # Bash benchmark
    bash_benchmark.md
    bash_ground_truth.yml
    apps/
```

## Language Benchmarks

### Go Benchmark v0.1

**256 test cases** across 12 CWE categories:

| Category | CWE | Description | Vuln | Safe | Total |
|----------|-----|-------------|------|------|-------|
| sqli | 89 | SQL Injection | 25 | 25 | 50 |
| cmdi | 78 | OS Command Injection | 13 | 17 | 30 |
| pathtraver | 22 | Path Traversal | 16 | 14 | 30 |
| xss | 79 | Cross-Site Scripting | 10 | 10 | 20 |
| ssrf | 918 | Server-Side Request Forgery | 10 | 10 | 20 |
| weakrand | 330 | Weak Random Number Generation | 10 | 10 | 20 |
| weakhash | 328 | Weak Hashing Algorithm | 10 | 10 | 20 |
| weakcipher | 327 | Weak Encryption Algorithm | 8 | 8 | 16 |
| securecookie | 614 | Missing Cookie Security Flags | 8 | 8 | 16 |
| redirect | 601 | Open Redirect | 8 | 8 | 16 |
| tlsverify | 295 | Improper Certificate Validation | 5 | 5 | 10 |
| deserial | 502 | Insecure Deserialization | 4 | 4 | 8 |

**Frameworks tested:** net/http, gin, chi, echo, fiber, gorilla/mux, beego, gRPC

### Rust Benchmark v0.2

**98 test cases** across 13 CWE categories, 8 real applications, 4 web frameworks:

| Category | CWE | Description | Vuln | Safe | Total |
|----------|-----|-------------|------|------|-------|
| sqli | 89 | SQL Injection | 20 | 15 | 35 |
| cmdi | 78 | OS Command Injection | 11 | 0 | 11 |
| pathtraver | 22 | Path Traversal | 11 | 1 | 12 |
| memsafety | 119 | Memory Safety (unsafe) | 7 | 4 | 11 |
| ssrf | 918 | Server-Side Request Forgery | 7 | 0 | 7 |
| crypto | 327 | Weak Cryptography | 5 | 0 | 5 |
| infodisclosure | 200 | Information Disclosure | 5 | 0 | 5 |
| weakrand | 330 | Weak Random | 3 | 0 | 3 |
| xss | 79 | Cross-Site Scripting | 2 | 0 | 2 |
| deser | 502 | Insecure Deserialization | 2 | 0 | 2 |
| intoverflow | 190 | Integer Overflow | 2 | 0 | 2 |
| redos | 1333 | ReDoS | 1 | 0 | 1 |
| inputval | 20 | Missing Input Validation | 1 | 0 | 1 |

**Frameworks tested:** actix-web, axum, Rocket, Warp

**Applications:** 8 apps ranging from intentional taint test apps to real production-style codebases (calorie tracker, job queue, microservices). Includes both intentionally vulnerable code and naturally-occurring bugs with weak escaping patterns.

**Known limitation:** TN (safe) coverage is sparse outside of SQL injection and memory safety. Categories with 0 safe test cases can only measure recall (TPR), not precision (FPR). See `rust/CHANGELOG.md` for details.

### Bash Benchmark v0.1

_(In progress -- see bash/ directory)_

## How to Use

### 1. Point your SAST tool at a language directory

```bash
# Example with TheAuditor
cd gorustbash_benchmark/go
aud full --offline
```

### 2. Run the scoring script

Each language directory contains a `*_benchmark.md` file with a ready-to-run scoring script. The script reads findings from the SAST tool's database and compares against `expectedresults-0.1.csv`.

### 3. Read the scorecard

```
Category         CWE    TP    FP    FN    TN      TPR     FPR   Score
----------------------------------------------------------------------
sqli             89     X     X     X     X     XX.X%   XX.X%  +XX.X%
cmdi             78     X     X     X     X     XX.X%   XX.X%  +XX.X%
...
----------------------------------------------------------------------
OVERALL                 X     X     X     X     XX.X%   XX.X%  +XX.X%
```

### 4. Analyze gaps

Every FN reveals a detection gap. Every FP reveals a false positive source. Root-cause each one to improve your tool.

## Design Principles

1. **Unbiased test cases** -- Written from security knowledge, not from knowledge of any specific tool's detection capabilities.

2. **No vulnerability hints** -- Test source code contains no comments like "VULNERABLE" or "TAINT SINK". The CSV answer key is the only ground truth.

3. **Safe code looks like vulnerable code** -- The hardest FP test cases use patterns that resemble vulnerabilities but are properly sanitized, validated, or use parameterized APIs.

4. **Real-world patterns** -- Test cases are based on actual CVE patterns, Go/Rust/Bash security advisories, and common developer mistakes in production codebases.

5. **Framework diversity** -- Tests cover multiple web frameworks per language to validate framework-aware detection.

6. **~50/50 split** -- Each category aims for equal numbers of vulnerable and safe test cases. Both TPR and FPR matter.

## CSV Format

```csv
# test name,category,real vulnerability,CWE
BenchmarkTest00001,sqli,true,89
BenchmarkTest00002,sqli,false,89
```

## Inspiration

This project is directly inspired by:

- [OWASP Benchmark for Java](https://owasp.org/www-project-benchmark/) -- 2,740 test cases, the gold standard
- [OWASP Benchmark for Python](https://github.com/owasp/benchmark-python) -- 1,230 test cases
- [OWASP Juice Shop](https://owasp.org/www-project-juice-shop/) -- Node.js intentionally vulnerable app with challenge annotations

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on adding test cases, reporting issues, and improving the benchmark.

## License

This project is licensed under the Apache License 2.0 -- see [LICENSE](LICENSE) for details.
