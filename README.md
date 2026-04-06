# OWASP-Style SAST Benchmark for Go, Rust, Bash, PHP, Ruby + Adversarial Evasion + Chain Detection

The first public Static Application Security Testing (SAST) benchmark suite for **Go**, **Rust**, **Bash**, **PHP**, and **Ruby** -- languages with zero existing public SAST benchmarks. Plus an **Adversarial Evasion** benchmark that tests whether your tool can detect that someone is *hiding* something, and a **Chain Detection** benchmark that tests whether your tool can correlate multiple findings into compound exploit paths.

## Why This Exists

SAST tools need ground truth to measure accuracy. Without benchmarks, you can't answer:

- **Does the tool actually detect this vulnerability?** (True Positive Rate)
- **Does the tool cry wolf on safe code?** (False Positive Rate)
- **Is the tool getting better or worse over time?** (Regression detection)

The [OWASP Benchmark for Java](https://owasp.org/www-project-benchmark/) (2,740 test cases) and [OWASP Benchmark for Python](https://github.com/OWASP-Benchmark/BenchmarkPython) (1,230 test cases) proved this model works. No equivalent exists for Go, Rust, Bash, PHP, or Ruby. This project fills that gap.

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
    validate_chains.py       # Chain detection CSV/annotation consistency checker
  go/                      # Go benchmark (534 tests, 24 CWEs)
    expectedresults-0.3.2.csv
    go_benchmark.md
    SCORING.md               # Scoring methodology and tool instructions
    CHANGELOG.md
    testcode/
    apps/
  rust/                    # Rust benchmark (491 tests, 20 CWEs)
    expectedresults-0.4.0.csv  # Answer key (scoring authority)
    rust_benchmark.md        # Scoring script + methodology
    CHANGELOG.md
    dev_roadmap.md
    testcode/                # 149 standalone test files
    apps/                    # 8 annotated applications
  bash/                    # Bash benchmark (526 tests, 20 CWEs)
    expectedresults-0.4.0.csv  # Answer key (526 test cases, OWASP CSV format)
    bash_benchmark.py        # Automated scoring script
    bash_benchmark.md        # Methodology, engine analysis, scorecard
    CHANGELOG.md             # Version history
    apps/                    # 5 annotated applications
    testcode/                # 20 standalone CWE test files
  php/                     # PHP benchmark (369 tests, 25 CWEs)
    expectedresults-0.1.0.csv  # Answer key (369 test cases, OWASP CSV format)
    php_benchmark.md         # Full benchmark specification
    SCORING.md               # Scoring methodology and tool instructions
    CHANGELOG.md             # Version history
    testcode/                # 251 standalone test files
    apps/                    # 4 annotated applications
  ruby/                    # Ruby benchmark (318 tests, 25 CWEs)
    expectedresults-0.1.0.csv  # Answer key (318 test cases, OWASP CSV format)
    ruby_benchmark.md        # Full benchmark specification
    SCORING.md               # Scoring methodology and tool instructions
    CHANGELOG.md             # Version history
    testcode/                # 256 standalone test files
    apps/                    # 3 annotated applications (rack_app, rails_api, sinatra_app)
  adversarial/             # Adversarial evasion benchmark (123 tests, 10 categories)
    expectedresults-0.2.0.csv  # Answer key (123 test cases, OWASP CSV format)
    baseline_theauditor_tool_score.md  # TheAuditor baseline results
    adversarial_benchmark.md   # Methodology and category descriptions
    CHANGELOG.md               # Version history
    testcode/                  # Cross-language test cases (JS, Python, Go)
  chains/                    # Chain detection benchmark (16 tests, 4 categories)
    expectedresults-0.1.0.csv  # Answer key (16 test cases, OWASP CSV format)
    baseline_theauditor_tool_score.md  # TheAuditor baseline results
    chain_benchmark.md         # Methodology and scenario descriptions
    CHANGELOG.md               # Version history
    scenarios/                 # Multi-file Flask applications (vuln/safe pairs)
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

### Rust v0.4.0 -- 491 test cases, 20 CWEs, 4 frameworks

| Category | CWE | Vuln | Safe | Total |
|----------|-----|------|------|-------|
| sqli | 89 | 23 | 27 | 50 |
| cmdi | 78 | 14 | 16 | 30 |
| pathtraver | 22 | 14 | 14 | 28 |
| ssrf | 918 | 12 | 13 | 25 |
| xss | 79 | 12 | 12 | 24 |
| memsafety | 119 | 12 | 12 | 24 |
| crypto | 327 | 12 | 12 | 24 |
| weakrand | 330 | 12 | 12 | 24 |
| infodisclosure | 200 | 12 | 12 | 24 |
| deser | 502 | 12 | 12 | 24 |
| intoverflow | 190 | 12 | 12 | 24 |
| redos | 1333 | 12 | 12 | 24 |
| inputval | 20 | 12 | 12 | 24 |
| hardcodedcreds | 798 | 12 | 10 | 22 |
| race_condition | 362 | 10 | 10 | 20 |
| loginjection | 117 | 10 | 10 | 20 |
| securecookie | 614 | 10 | 10 | 20 |
| redirect | 601 | 10 | 10 | 20 |
| fileupload | 434 | 10 | 10 | 20 |
| tlsverify | 295 | 10 | 10 | 20 |

Frameworks: actix-web, axum, Rocket, Warp. 8 reference apps in `apps/` + ~372 standalone test files in `testcode/`. TP/TN balance: 49/51. All 20 categories have minimum 10 TP and 10 TN. v0.4.0: 7 new CWEs (hardcodedcreds, race_condition, loginjection, securecookie, redirect, fileupload, tlsverify), all existing categories expanded to 12/12, 2 CWE-798 reclassifications.

### Bash v0.4.0 -- 526 test cases, 20 CWEs

| Category | CWE | Vuln | Safe | Total |
|----------|-----|------|------|-------|
| cmdi | 78 | 53 | 53 | 106 |
| sqli | 89 | 21 | 21 | 42 |
| codeinj | 94 | 18 | 18 | 36 |
| ssrf | 918 | 11 | 11 | 22 |
| auth_bypass | 287 | 10 | 10 | 20 |
| cleartext_tx | 319 | 10 | 10 | 20 |
| dos | 770 | 10 | 10 | 20 |
| hardcoded_creds | 798 | 10 | 10 | 20 |
| infodisclosure | 200 | 10 | 10 | 20 |
| insecure_perms | 732 | 10 | 10 | 20 |
| insecure_temp | 377 | 10 | 10 | 20 |
| loginjection | 117 | 10 | 10 | 20 |
| pathtraver | 22 | 10 | 10 | 20 |
| privilege_escalation | 250 | 10 | 10 | 20 |
| race_condition | 362 | 10 | 10 | 20 |
| rce | 94 | 10 | 10 | 20 |
| ssl_bypass | 295 | 10 | 10 | 20 |
| unquoted | 78 | 10 | 10 | 20 |
| weakrand | 330 | 10 | 10 | 20 |
| weakcrypto | 327 | 10 | 10 | 20 |

5 applications: DevOps pipeline manager (10 scripts), HTTP webhook server (8 files), operations suite with SAFE_MODE toggle (7 files), data pipeline backup/deploy/healthcheck (4 files), CI/CD pipeline (7 files, TN-only). Plus 20 standalone CWE test files. TP/TN split: 50/50.

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

### Ruby v0.1.0 -- 318 test cases, 25 CWEs, 3 frameworks

| Category | CWE | Vuln | Safe | Total |
|----------|-----|------|------|-------|
| sqli | 89 | 19 | 19 | 38 |
| xss | 79 | 14 | 14 | 28 |
| cmdi | 78 | 12 | 12 | 24 |
| pathtraver | 22 | 9 | 9 | 18 |
| deserial | 502 | 8 | 8 | 16 |
| massassign | 915 | 7 | 7 | 14 |
| codeinj | 94 | 6 | 6 | 12 |
| csrf | 352 | 6 | 6 | 12 |
| fileupload | 434 | 6 | 6 | 12 |
| hardcodedcreds | 798 | 6 | 6 | 12 |
| redirect | 601 | 6 | 6 | 12 |
| ssrf | 918 | 6 | 6 | 12 |
| ssti | 1336 | 6 | 6 | 12 |
| weakhash | 328 | 6 | 6 | 12 |
| fileinclusion | 98 | 5 | 5 | 10 |
| securecookie | 614 | 5 | 5 | 10 |
| dynmethod | 94 | 4 | 4 | 8 |
| headerinj | 113 | 4 | 4 | 8 |
| unsafereflect | 470 | 4 | 4 | 8 |
| weakrand | 330 | 4 | 4 | 8 |
| xxe | 611 | 4 | 4 | 8 |
| ldapi | 90 | 3 | 3 | 6 |
| loginjection | 117 | 3 | 3 | 6 |
| regex | 1333 | 3 | 3 | 6 |
| weakcipher | 327 | 3 | 3 | 6 |

256 standalone test files in `testcode/` + 3 annotated applications (rack_app, rails_api, sinatra_app). Frameworks: Raw Ruby/Rack, Rails 7, Sinatra 3. TP/TN balance: 50/50. Ruby-specific CWEs: mass assignment via strong params bypass (CWE-915), dynamic method dispatch (CWE-94), SSTI via ERB.new (CWE-1336), unsafe reflection via const_get/send (CWE-470), ReDoS (CWE-1333).

### Adversarial Evasion v0.2.0 -- 123 test cases, 10 categories, cross-language

**This is not a language benchmark. It is a detection paradigm benchmark.**

While Go/Rust/Bash/PHP benchmarks ask "can your tool find this vulnerability?", the adversarial benchmark asks: **"can your tool detect that someone is hiding something?"** Traditional SAST is structurally blind to these attack classes because the AST, regex, and taint analysis all see "normal" code.

| Category | CWE | Vuln | Safe | Total | Detection Requires |
|----------|-----|------|------|-------|--------------------|
| unicode_payload | 506 | 5 | 5 | 10 | Byte-level scan |
| visual_deception | 451 | 6 | 4 | 10 | Confusables DB + Bidi detection |
| dynamic_construction | 506 | 6 | 4 | 10 | Taint through decode chains |
| supply_chain | 506 | 8 | 8 | 16 | File context + intent analysis |
| ai_prompt_injection | 1059 | 13 | 13 | 26 | NLP-level + config file scan |
| c2_fingerprint | 506 | 5 | 5 | 10 | Unusual API pattern detection |
| charset_mapping | 838 | 5 | 5 | 10 | CP mapping DB + byte scan |
| steganographic_payload | 506 | 5 | 5 | 10 | Binary->exec pattern detection |
| slopsquatting | 829 | 5 | 5 | 10 | Package existence DB |
| llm_code_generation | 506 | 4 | 4 | 8 | LLM response->exec pattern |

Cross-language (JavaScript, Python, Go). Based on real-world campaigns: Glassworm (March 2026), Trojan Source (CVE-2021-42574), os-info-checker-es6 (May 2025), WorstFit (CVE-2024-4577), buildrunner-dev (February 2026), Shai-Hulud v2 (November 2025), Unit 42 LLM assembly (January 2026), and the Trend Micro slopsquatting dataset (March 2025). TP/TN balance: 52/48.

See [adversarial/adversarial_benchmark.md](adversarial/adversarial_benchmark.md) for the full methodology, detection requirements matrix, and design philosophy.

### Chain Detection v0.1.0 -- 16 test cases, 4 categories, 8 multi-file scenarios

**This is not a vulnerability benchmark. It is a compositional reasoning benchmark.**

While all other benchmarks test whether a tool can detect a single finding, this benchmark tests: **"can your tool correlate multiple low-severity findings across files into a compound exploit path whose severity exceeds any individual component?"** No public benchmark for this capability exists. This is the first.

| Category | Scenarios | Vuln | Safe | Total | Chain Pattern |
|----------|-----------|------|------|-------|---------------|
| unauth_injection | 2 | 2 | 2 | 4 | Auth bypass + injection = unauthenticated attack |
| ssrf_pivot | 2 | 2 | 2 | 4 | SSRF + internal service = network boundary bypass |
| compound_injection | 2 | 2 | 2 | 4 | Injection A enables injection B |
| multi_stage | 2 | 2 | 2 | 4 | Multi-hop attack requiring 2+ vuln types |

Each scenario is a multi-file Flask application with `vuln/` (exploitable chain) and `safe/` (mitigated chain) variants. Safe variants differ by exactly one file. Based on real-world patterns: Capital One (2019), CVE-2023-22515, CVE-2024-23897, CVE-2015-7501.

See [chains/chain_benchmark.md](chains/chain_benchmark.md) for the full methodology, scenario descriptions, and safe variant design rules.

## Combined Scale

| Benchmark | Tests | CWEs/Categories | TP/TN Balance | What It Tests |
|-----------|-------|-----------------|---------------|---------------|
| Go | 534 | 24 CWEs | 50/50 | Vulnerability detection |
| PHP | 369 | 25 CWEs | 50/50 | Vulnerability detection |
| Bash | 526 | 20 CWEs | 50/50 | Vulnerability detection |
| Ruby | 318 | 25 CWEs | 50/50 | Vulnerability detection |
| Rust | 491 | 20 CWEs | 49/51 | Vulnerability detection |
| Adversarial | 123 | 10 categories | 52/48 | Evasion/concealment detection |
| Chains | 16 | 4 categories | 50/50 | Compound exploit chain detection |
| **Total** | **2,377** | **77 unique CWEs + 14 detection categories** | |

## How to Use

Score any SAST tool that exports SARIF 2.1.0 (Checkmarx, Semgrep, CodeQL, Snyk, etc.):

```bash
# 1. Run your tool and export SARIF
your-tool scan <benchmark_dir> --format sarif -o results.sarif

# 2. Score against ground truth
python scripts/score_sarif.py results.sarif <benchmark_dir>/expectedresults-*.csv
```

**Language benchmarks (Go/Rust/Bash/PHP/Ruby):**
```bash
your-tool scan go/testcode/ --format sarif -o results.sarif
python scripts/score_sarif.py results.sarif go/expectedresults-0.3.2.csv
```

**Adversarial evasion benchmark:**
```bash
your-tool scan adversarial/testcode/ --format sarif -o results.sarif
python scripts/score_sarif.py results.sarif adversarial/expectedresults-0.2.0.csv
```

**Chain detection benchmark:**
```bash
your-tool scan chains/scenarios/ --format sarif -o results.sarif
python scripts/score_sarif.py results.sarif chains/expectedresults-0.1.0.csv
```

Root-cause every FN (missed vulnerability) and FP (false alarm). That's where the benchmark earns its value.

## Limitations

Known limitations:

- **Classification accuracy**: Verified to our best ability. Community review welcome. Some edge cases may be debatable.
- **Scale**: OWASP Java has 2,740 tests. We have 1,984 across five languages + adversarial + chains. Growing with each release.
- **Self-graded**: We wrote the tests and the answer key. Independent verification is the next milestone.
- **Adversarial coverage**: 123 test cases across 10 categories. Dependency confusion (registry-level) and true AI polymorphic malware (runtime) are not yet covered as they require infrastructure beyond static source files.
- **Chain coverage**: 16 test cases across 4 categories. Python/Flask only for v0.1.0. Cross-language and longer chains planned for future releases.

We release this openly because imperfect ground truth that invites correction is more valuable than no ground truth at all.

## Inspiration

- [OWASP Benchmark for Java](https://owasp.org/www-project-benchmark/) -- 2,740 test cases
- [OWASP Benchmark for Python](https://github.com/OWASP-Benchmark/BenchmarkPython) -- 1,230 test cases
- [OWASP Juice Shop](https://owasp.org/www-project-juice-shop/) -- Node.js intentionally vulnerable app

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Apache License 2.0 -- see [LICENSE](LICENSE).
