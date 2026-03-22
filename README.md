# OWASP-Style SAST Benchmark for Go, Rust, and Bash

The first public Static Application Security Testing (SAST) benchmark suite for **Go**, **Rust**, and **Bash** -- three languages with zero existing public SAST benchmarks.

## Why This Exists

SAST tools need ground truth to measure accuracy. Without benchmarks, you can't answer:

- **Does the tool actually detect this vulnerability?** (True Positive Rate)
- **Does the tool cry wolf on safe code?** (False Positive Rate)
- **Is the tool getting better or worse over time?** (Regression detection)

The [OWASP Benchmark for Java](https://owasp.org/www-project-benchmark/) (2,740 test cases) and [OWASP Benchmark for Python](https://github.com/OWASP-Benchmark/BenchmarkPython) (1,230 test cases) proved this model works. No equivalent exists for Go, Rust, or Bash. This project fills that gap.

## Project Status

This benchmark is under active development and released as open source to invite community contribution and iteration. Building ground truth for three languages simultaneously is a massive undertaking. We have done our best to classify every test case correctly, but acknowledge that some classifications may need adjustment as the community reviews them. That is the point of open-sourcing it -- no single team can perfectly write their own exam and grade it too.

If you find a misclassification, please open an issue. Every correction makes the benchmark more valuable for everyone.

## Scoring Methodology

Each test case is classified as either **vulnerable** (true) or **safe** (false) in a ground truth file. After running a SAST tool, scoring computes:

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
  README.md               # This file
  LICENSE                  # Apache 2.0
  CONTRIBUTING.md          # How to contribute
  scripts/                 # Tool-agnostic scoring scripts
    score_sarif.py           # SARIF scorer (any SAST tool)
    convert_theauditor.py    # TheAuditor DB to SARIF bridge
    validate_go.py           # Go CSV/file consistency checker
  go/                      # Go benchmark (476 tests, 21 CWEs)
    expectedresults-0.1.csv
    go_benchmark.md
    SCORING.md               # Scoring methodology and tool instructions
    CHANGELOG.md
    testcode/
    apps/
  rust/                    # Rust benchmark (262 tests, 13 CWEs)
    expectedresults-0.1.csv  # Answer key (scoring authority)
    rust_benchmark.md        # Scoring script + methodology
    CHANGELOG.md
    dev_roadmap.md
    testcode/                # 144 standalone test files
    apps/                    # 8 annotated applications
  bash/                    # Bash benchmark (356 tests, 16 CWEs)
    bash_ground_truth.yml    # Answer key (356 test cases)
    bash_benchmark.py        # Automated scoring script
    BENCHMARK.md             # Methodology, engine analysis, scorecard
    CHANGELOG.md             # Version history
    apps/                    # 4 annotated applications
    testcode/                # 16 standalone CWE test files
```

## Language Benchmarks

### Go v0.3.1 -- 476 test cases, 21 CWEs, 8 frameworks

| Category | CWE | Vuln | Safe | Total |
|----------|-----|------|------|-------|
| sqli | 89 | 65 | 58 | 123 |
| cmdi | 78 | 30 | 30 | 60 |
| pathtraver | 22 | 25 | 30 | 55 |
| xss | 79 | 13 | 15 | 28 |
| ssrf | 918 | 10 | 10 | 20 |
| weakrand | 330 | 10 | 10 | 20 |
| weakhash | 328 | 10 | 10 | 20 |
| weakcipher | 327 | 8 | 8 | 16 |
| securecookie | 614 | 8 | 8 | 16 |
| redirect | 601 | 8 | 8 | 16 |
| hardcodedcreds | 798 | 6 | 6 | 12 |
| authnfailure | 287 | 6 | 6 | 12 |
| tlsverify | 295 | 5 | 5 | 10 |
| loginjection | 117 | 4 | 5 | 9 |
| nosql | 943 | 4 | 5 | 9 |
| authzfailure | 862 | 5 | 4 | 9 |
| csrf | 352 | 5 | 4 | 9 |
| codeinj | 94 | 4 | 4 | 8 |
| ldapi | 90 | 4 | 4 | 8 |
| trustbound | 501 | 4 | 4 | 8 |
| deserial | 502 | 4 | 4 | 8 |

Plus 5 reference apps with 395 classified functions. Frameworks: net/http, gin, chi, echo, fiber, gorilla/mux, beego, gRPC. Tool-agnostic SARIF-based scoring. Includes OWASP-style discrimination patterns, cross-file flows, GORM/sqlx/syscall/WebSocket/zip-slip patterns.

### Rust v0.3.1 -- 262 test cases, 13 CWEs, 4 frameworks

| Category | CWE | Vuln | Safe | Total |
|----------|-----|------|------|-------|
| sqli | 89 | 24 | 26 | 50 |
| cmdi | 78 | 14 | 16 | 30 |
| pathtraver | 22 | 14 | 14 | 28 |
| ssrf | 918 | 9 | 13 | 22 |
| memsafety | 119 | 8 | 12 | 20 |
| crypto | 327 | 9 | 11 | 20 |
| weakrand | 330 | 7 | 9 | 16 |
| xss | 79 | 5 | 11 | 16 |
| infodisclosure | 200 | 8 | 8 | 16 |
| deser | 502 | 6 | 6 | 12 |
| intoverflow | 190 | 5 | 7 | 12 |
| redos | 1333 | 5 | 5 | 10 |
| inputval | 20 | 4 | 6 | 10 |

Frameworks: actix-web, axum, Rocket, Warp. 8 reference apps in `apps/` + 143 standalone test files in `testcode/`. TP/TN balance: 45/55. All 13 categories have both vulnerable and safe test cases. v0.3.1 fixed 12 misclassified test cases identified during OWASP Foundation review.

### Bash v0.3.1 -- 356 test cases, 16 CWEs

| Category | CWE | Vuln | Safe | Total |
|----------|-----|------|------|-------|
| cmdi | 78 | 53 | 53 | 106 |
| sqli | 89 | 21 | 21 | 42 |
| codeinj | 94 | 18 | 18 | 36 |
| ssrf | 918 | 11 | 11 | 22 |
| unquoted | 78 | 10 | 10 | 20 |
| pathtraver | 22 | 9 | 9 | 18 |
| infodisclosure | 200 | 6 | 9 | 15 |
| hardcoded_creds | 798 | 7 | 7 | 14 |
| ssl_bypass | 295 | 6 | 7 | 13 |
| weakcrypto | 327 | 6 | 6 | 12 |
| insecure_perms | 732 | 5 | 7 | 12 |
| rce | 94 | 5 | 5 | 10 |
| weakrand | 330 | 5 | 5 | 10 |
| race_condition | 362 | 5 | 5 | 10 |
| insecure_temp | 377 | 4 | 4 | 8 |
| auth_bypass | 287 | 4 | 4 | 8 |

5 applications: DevOps pipeline manager (10 scripts), HTTP webhook server (8 files), operations suite with SAFE_MODE toggle (7 files), data pipeline backup/deploy/healthcheck (4 files), hardened CI/CD pipeline (7 files, safe-only). Plus 13 adversarial CWE test files. TP/TN split: 49/51.

## Combined Scale

| Language | Tests | CWEs | TP/TN Balance |
|----------|-------|------|---------------|
| Go | 476 | 21 | 50/50 |
| Bash | 356 | 16 | 49/51 |
| Rust | 262 | 13 | 44/56 |
| **Total** | **1,066** | **47 unique** | |

## How to Use

1. Point your SAST tool at a language directory
2. Run the scoring script from the language's benchmark doc
3. Compare against ground truth
4. Root-cause every FN and FP

## Limitations

This is v0.3. Known limitations:

- **Classification accuracy**: Verified to our best ability. Community review welcome. Some edge cases may be debatable.
- **Scale**: OWASP Java has 2,740 tests. We have 923 across three languages. Growing with each release.
- **Safe variant coverage**: Some categories (especially Rust/Bash) have fewer safe test cases, limiting FPR measurement.
- **Self-graded**: We wrote the tests and the answer key. Independent verification is the next milestone.

We release this openly because imperfect ground truth that invites correction is more valuable than no ground truth at all.

## Inspiration

- [OWASP Benchmark for Java](https://owasp.org/www-project-benchmark/) -- 2,740 test cases
- [OWASP Benchmark for Python](https://github.com/OWASP-Benchmark/BenchmarkPython) -- 1,230 test cases
- [OWASP Juice Shop](https://owasp.org/www-project-juice-shop/) -- Node.js intentionally vulnerable app

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Apache License 2.0 -- see [LICENSE](LICENSE).
