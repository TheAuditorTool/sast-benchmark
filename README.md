# OWASP-Style SAST Benchmark for Go, Rust, Bash, PHP + Adversarial Evasion

The first public Static Application Security Testing (SAST) benchmark suite for **Go**, **Rust**, **Bash**, and **PHP** -- languages with zero existing public SAST benchmarks. Plus an **Adversarial Evasion** benchmark that tests an entirely different question: can your tool detect that someone is *hiding* something?

## Why This Exists

SAST tools need ground truth to measure accuracy. Without benchmarks, you can't answer:

- **Does the tool actually detect this vulnerability?** (True Positive Rate)
- **Does the tool cry wolf on safe code?** (False Positive Rate)
- **Is the tool getting better or worse over time?** (Regression detection)

The [OWASP Benchmark for Java](https://owasp.org/www-project-benchmark/) (2,740 test cases) and [OWASP Benchmark for Python](https://github.com/OWASP-Benchmark/BenchmarkPython) (1,230 test cases) proved this model works. No equivalent exists for Go, Rust, Bash, or PHP. This project fills that gap.

### Beyond Traditional SAST: Adversarial Evasion

Traditional SAST benchmarks measure whether a tool can follow tainted data from source to sink. The **Adversarial Evasion** benchmark measures something no existing benchmark tests: can your tool detect **intentional concealment**?

This matters because the real-world threat landscape has shifted. Attackers now hide malicious payloads inside invisible Unicode characters (the Glassworm campaign compromised 433 repos in March 2026), use Bidi overrides to make code *display* differently than it *executes* (Trojan Source, CVE-2021-42574), embed C2 channels in Google Calendar events and Solana transaction memos, and inject adversarial prompts targeting AI coding assistants. None of these attacks involve a traditional taint flow. AST analysis, regex matching, and dataflow tracking all see "normal" code. Detection requires new paradigms: byte-level Unicode scanning, visual deception analysis, behavioral intent modeling, and supply chain anomaly detection.

The adversarial benchmark provides the first public ground truth for measuring these capabilities. See [adversarial/adversarial_benchmark.md](adversarial/adversarial_benchmark.md) for the full specification.

## Project Status

This benchmark is under active development and released as open source to invite community contribution and iteration. Building ground truth for four languages simultaneously is a massive undertaking. We have done our best to classify every test case correctly, but acknowledge that some classifications may need adjustment as the community reviews them. That is the point of open-sourcing it -- no single team can perfectly write their own exam and grade it too.

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
    validate_php.py          # PHP CSV/annotation consistency checker
    validate_adversarial.py  # Adversarial CSV/annotation consistency checker
  go/                      # Go benchmark (534 tests, 24 CWEs)
    expectedresults-0.3.2.csv
    go_benchmark.md
    SCORING.md               # Scoring methodology and tool instructions
    CHANGELOG.md
    testcode/
    apps/
  rust/                    # Rust benchmark (268 tests, 13 CWEs)
    expectedresults-0.3.2.csv  # Answer key (scoring authority)
    rust_benchmark.md        # Scoring script + methodology
    CHANGELOG.md
    dev_roadmap.md
    testcode/                # 144 standalone test files
    apps/                    # 8 annotated applications
  bash/                    # Bash benchmark (356 tests, 16 CWEs)
    expectedresults-0.3.2.csv  # Answer key (356 test cases, OWASP CSV format)
    bash_benchmark.py        # Automated scoring script
    BENCHMARK.md             # Methodology, engine analysis, scorecard
    CHANGELOG.md             # Version history
    apps/                    # 4 annotated applications
    testcode/                # 16 standalone CWE test files
  php/                     # PHP benchmark (369 tests, 25 CWEs)
    expectedresults-0.1.0.csv  # Answer key (369 test cases, OWASP CSV format)
    php_benchmark.md         # Full benchmark specification
    SCORING.md               # Scoring methodology and tool instructions
    CHANGELOG.md             # Version history
    testcode/                # 251 standalone test files
    apps/                    # 4 annotated applications
  adversarial/             # Adversarial evasion benchmark (60 tests, 6 categories)
    expectedresults-0.1.0.csv  # Answer key (60 test cases, OWASP CSV format)
    adversarial_benchmark.py   # Scoring script (EIDL-aware)
    adversarial_benchmark.md   # Methodology and category descriptions
    CHANGELOG.md               # Version history
    testcode/                  # Cross-language test cases (JS, Python, Go)
```

## Language Benchmarks

### Go v0.3.2 -- 534 test cases, 24 CWEs, 8 frameworks

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
| trustbound | 501 | 6 | 6 | 12 |
| ldapi | 90 | 6 | 6 | 12 |
| deserial | 502 | 6 | 6 | 12 |
| codeinj | 94 | 6 | 6 | 12 |
| loginjection | 117 | 6 | 7 | 13 |
| nosql | 943 | 6 | 7 | 13 |
| authzfailure | 862 | 7 | 6 | 13 |
| csrf | 352 | 7 | 6 | 13 |
| tlsverify | 295 | 5 | 5 | 10 |
| race_condition | 362 | 5 | 5 | 10 |
| fileupload | 434 | 4 | 4 | 8 |
| inputval | 20 | 4 | 4 | 8 |

Plus 5 reference apps with 395 classified functions. Frameworks: net/http, gin, chi, echo, fiber, gorilla/mux, beego, gRPC. Tool-agnostic SARIF-based scoring. Includes OWASP-style discrimination patterns, cross-file flows, GORM/sqlx/syscall/WebSocket/zip-slip patterns.

### Rust v0.3.2 -- 268 test cases, 13 CWEs, 4 frameworks

| Category | CWE | Vuln | Safe | Total |
|----------|-----|------|------|-------|
| sqli | 89 | 24 | 26 | 50 |
| cmdi | 78 | 14 | 16 | 30 |
| pathtraver | 22 | 14 | 14 | 28 |
| ssrf | 918 | 9 | 13 | 22 |
| memsafety | 119 | 8 | 12 | 20 |
| crypto | 327 | 9 | 11 | 20 |
| weakrand | 330 | 7 | 9 | 16 |
| xss | 79 | 11 | 11 | 22 |
| infodisclosure | 200 | 8 | 8 | 16 |
| deser | 502 | 6 | 6 | 12 |
| intoverflow | 190 | 5 | 7 | 12 |
| redos | 1333 | 5 | 5 | 10 |
| inputval | 20 | 4 | 6 | 10 |

Frameworks: actix-web, axum, Rocket, Warp. 8 reference apps in `apps/` + 149 standalone test files in `testcode/`. TP/TN balance: 46/54. All 13 categories have both vulnerable and safe test cases. v0.3.2: XSS rebalanced to 50/50, all source code hints stripped, SARIF scoring consolidated.

### Bash v0.3.2 -- 356 test cases, 16 CWEs

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

5 applications: DevOps pipeline manager (10 scripts), HTTP webhook server (8 files), operations suite with SAFE_MODE toggle (7 files), data pipeline backup/deploy/healthcheck (4 files), CI/CD pipeline (7 files, TN-only). Plus 13 adversarial CWE test files. TP/TN split: 49/51.

### PHP v0.1.0 -- 369 test cases, 25 CWEs, 4 frameworks

| Category | CWE | Vuln | Safe | Total |
|----------|-----|------|------|-------|
| sqli | 89 | 28 | 28 | 56 |
| xss | 79 | 20 | 19 | 39 |
| cmdi | 78 | 12 | 12 | 24 |
| pathtraver | 22 | 12 | 11 | 23 |
| fileinclusion | 98 | 10 | 9 | 19 |
| typejuggling | 697 | 7 | 7 | 14 |
| ssrf | 918 | 7 | 7 | 14 |
| deserial | 502 | 7 | 7 | 14 |
| codeinj | 94 | 6 | 6 | 12 |
| csrf | 352 | 6 | 6 | 12 |
| fileupload | 434 | 6 | 6 | 12 |
| hardcodedcreds | 798 | 6 | 6 | 12 |
| xxe | 611 | 6 | 6 | 12 |
| extract | 621 | 5 | 5 | 10 |
| massassign | 915 | 5 | 5 | 10 |
| redirect | 601 | 5 | 5 | 10 |
| ssti | 1336 | 5 | 5 | 10 |
| weakhash | 328 | 5 | 5 | 10 |
| weakrand | 330 | 5 | 5 | 10 |
| headerinj | 113 | 4 | 4 | 8 |
| ldapi | 90 | 4 | 4 | 8 |
| securecookie | 614 | 4 | 4 | 8 |
| unsafereflect | 470 | 4 | 4 | 8 |
| variablevars | 627 | 4 | 4 | 8 |
| weakcipher | 327 | 3 | 3 | 6 |

251 standalone test files in `testcode/` + 4 annotated applications (vuln_blog, laravel_api, wp_plugin, symfony_app). Frameworks: Raw PHP/PDO, Laravel, WordPress, Symfony. Covers both modern PHP 8.x and legacy PHP 5.x/7.x patterns. TP/TN balance: 50/50. 6 PHP-unique CWEs not found in any other benchmark: file inclusion (CWE-98), type juggling (CWE-697), variable extraction (CWE-621), variable variables (CWE-627), unsafe reflection (CWE-470), SSTI (CWE-1336).

### Adversarial Evasion v0.1.0 -- 60 test cases, 6 categories, cross-language

**This is not a language benchmark. It is a detection paradigm benchmark.**

While Go/Rust/Bash/PHP benchmarks ask "can your tool find this vulnerability?", the adversarial benchmark asks: **"can your tool detect that someone is hiding something?"** Traditional SAST is structurally blind to these attack classes because the AST, regex, and taint analysis all see "normal" code.

| Category | CWE | Vuln | Safe | Total | Detection Requires |
|----------|-----|------|------|-------|--------------------|
| unicode_payload | 506 | 5 | 5 | 10 | Byte-level scan |
| visual_deception | 451 | 6 | 4 | 10 | Confusables DB + Bidi detection |
| dynamic_construction | 506 | 6 | 4 | 10 | Taint through decode chains |
| supply_chain | 506 | 5 | 5 | 10 | File context + intent analysis |
| ai_prompt_injection | 1059 | 5 | 5 | 10 | NLP-level + tag-block scan |
| c2_fingerprint | 506 | 5 | 5 | 10 | Unusual API pattern detection |

Cross-language (JavaScript, Python, Go). Based on real-world campaigns: Glassworm (March 2026, 433 repos via Variation Selector encoding), os-info-checker-es6 (May 2025, VS steganography + Google Calendar C2), Trojan Source (CVE-2021-42574, Bidi overrides), and emerging AI prompt injection attacks targeting coding assistants. TP/TN balance: 53/47.

See [adversarial/adversarial_benchmark.md](adversarial/adversarial_benchmark.md) for the full methodology, detection requirements matrix, and design philosophy.

## Combined Scale

| Benchmark | Tests | CWEs/Categories | TP/TN Balance | What It Tests |
|-----------|-------|-----------------|---------------|---------------|
| Go | 534 | 24 CWEs | 50/50 | Vulnerability detection |
| PHP | 369 | 25 CWEs | 50/50 | Vulnerability detection |
| Bash | 356 | 16 CWEs | 49/51 | Vulnerability detection |
| Rust | 268 | 13 CWEs | 46/54 | Vulnerability detection |
| Adversarial | 60 | 6 categories | 53/47 | Evasion/concealment detection |
| **Total** | **1,587** | **66 unique CWEs + 6 evasion categories** | |

## How to Use

**Language benchmarks (Go/Rust/Bash/PHP):**
1. Point your SAST tool at a language directory
2. Export findings as SARIF (or use the tool-specific bridge script)
3. Run `python scripts/score_sarif.py <results.sarif> <language>/expectedresults-*.csv`
4. Root-cause every FN and FP

**Adversarial evasion benchmark:**
1. Point your tool at `adversarial/testcode/` (cross-language: JS, Python, Go)
2. Run `python adversarial/adversarial_benchmark.py`
3. Scoring uses the same Youden's J formula but maps findings through EIDL signal/rule/sink tracks
4. Root-cause every FN -- each one represents an evasion technique your tool is blind to

## Limitations

Known limitations:

- **Classification accuracy**: Verified to our best ability. Community review welcome. Some edge cases may be debatable.
- **Scale**: OWASP Java has 2,740 tests. We have 1,587 across four languages + adversarial. Growing with each release.
- **Self-graded**: We wrote the tests and the answer key. Independent verification is the next milestone.
- **Adversarial coverage**: The adversarial benchmark (60 tests) is intentionally smaller -- each test case represents a real-world attack pattern, not a synthetic variant. It does not yet cover slopsquatting (ecosystem-level), dependency confusion (registry-level), or AI polymorphic malware (runtime). These require fundamentally different testing infrastructure.

We release this openly because imperfect ground truth that invites correction is more valuable than no ground truth at all.

## Inspiration

- [OWASP Benchmark for Java](https://owasp.org/www-project-benchmark/) -- 2,740 test cases
- [OWASP Benchmark for Python](https://github.com/OWASP-Benchmark/BenchmarkPython) -- 1,230 test cases
- [OWASP Juice Shop](https://owasp.org/www-project-juice-shop/) -- Node.js intentionally vulnerable app

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Apache License 2.0 -- see [LICENSE](LICENSE).
