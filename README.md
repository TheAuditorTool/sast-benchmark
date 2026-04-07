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

This matters because real-world attacks have moved beyond taint flows. Attackers now hide malicious payloads inside invisible Unicode characters (the Glassworm campaign compromised 433 repos in March 2026), use Bidi overrides to make code *display* differently than it *executes* (Trojan Source, CVE-2021-42574), embed C2 channels in Google Calendar events and Solana transaction memos, and inject adversarial prompts targeting AI coding assistants. None of these attacks involve a traditional taint flow. AST analysis, regex matching, and dataflow tracking all see "normal" code. Catching them requires different techniques: byte-level Unicode scanning, visual deception analysis, behavioral intent modeling, and supply chain anomaly detection.

The adversarial benchmark provides the first public ground truth for measuring these capabilities. See [adversarial/adversarial_benchmark.md](adversarial/adversarial_benchmark.md) for the full specification.

## Project Status

This benchmark is under active development and released as open source to invite community contribution and iteration. Building ground truth for five languages simultaneously is a massive undertaking. We have done our best to classify every test case correctly, but acknowledge that some classifications may need adjustment as the community reviews them. That is the point of open-sourcing it -- no single team can perfectly write their own exam and grade it too.

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
    validate_php.py          # PHP filename-based fidelity + anti-leakage checker
    validate_adversarial.py  # Adversarial CSV/annotation consistency checker
    validate_chains.py       # Chain detection CSV/annotation consistency checker
  go/                      # Go benchmark (1,350 tests, 25 CWEs)
    expectedresults-0.5.1.csv
    go_benchmark.md
    SCORING.md
    CHANGELOG.md
    testcode/                # 1,350 standalone test files
  rust/                    # Rust benchmark (1,133 tests, 25 CWEs)
    expectedresults-0.5.2.csv
    rust_benchmark.md
    CHANGELOG.md
    dev_roadmap.md
    testcode/                # 1,133 standalone test files
  bash/                    # Bash benchmark (867 tests, 20 CWEs)
    expectedresults-0.5.3.csv
    bash_benchmark.md
    CHANGELOG.md
    testcode/                # 867 standalone test files
  php/                     # PHP benchmark (1,138 tests, 25 CWEs)
    expectedresults-0.3.2.csv
    php_benchmark.md
    SCORING.md
    CHANGELOG.md
    testcode/                # 1,138 standalone test files
  ruby/                    # Ruby benchmark (1,288 tests, 27 CWEs)
    expectedresults-0.3.2.csv
    ruby_benchmark.md
    SCORING.md
    CHANGELOG.md
    testcode/                # 1,288 standalone test files
  vulnerable_apps/         # 26 reference apps across 5 languages (884 ground truth entries)
    README.md                # Overview, scoring methodology, directory map
    go/                      # 6 apps (394 entries) -- per-app ground_truth.csv
    rust/                    # 8 apps (119 entries) -- annotation-based ground_truth.csv
    php/                     # 4 apps (118 entries) -- annotation-based ground_truth.csv
    bash/                    # 5 apps (191 entries) -- annotation-based ground_truth.csv
    ruby/                    # 3 apps (62 entries)  -- annotation-based ground_truth.csv
  adversarial/             # Adversarial evasion benchmark (123 tests, 10 categories)
    expectedresults-0.2.0.csv
    baseline_theauditor_tool_score.md
    adversarial_benchmark.md
    CHANGELOG.md
    testcode/                # Cross-language test cases (JS, Python, Go)
  chains/                  # Chain detection benchmark (500 tests, 20 categories)
    expectedresults-0.2.0.csv
    baseline_theauditor_tool_score.md
    chain_benchmark.md
    CHANGELOG.md
    scenarios/               # Multi-file Flask applications (vuln/safe pairs)
```

## Language Benchmarks

### Go v0.5.1 -- 1,350 test cases, 25 CWEs, 8 frameworks

| Category | CWE | Vuln | Safe | Total |
|----------|-----|------|------|-------|
| sqli | 89 | 65 | 65 | 130 |
| cmdi | 78 | 30 | 30 | 60 |
| pathtraver | 22 | 30 | 30 | 60 |
| xss | 79 | 25 | 25 | 50 |
| ssrf | 918 | 25 | 25 | 50 |
| weakrand | 330 | 25 | 25 | 50 |
| weakhash | 328 | 25 | 25 | 50 |
| weakcipher | 327 | 25 | 25 | 50 |
| securecookie | 614 | 25 | 25 | 50 |
| redirect | 601 | 25 | 25 | 50 |
| infodisclosure | 200 | 25 | 25 | 50 |
| hardcodedcreds | 798 | 25 | 25 | 50 |
| authnfailure | 287 | 25 | 25 | 50 |
| trustbound | 501 | 25 | 25 | 50 |
| ldapi | 90 | 25 | 25 | 50 |
| deserial | 502 | 25 | 25 | 50 |
| codeinj | 94 | 25 | 25 | 50 |
| loginjection | 117 | 25 | 25 | 50 |
| nosql | 943 | 25 | 25 | 50 |
| authzfailure | 862 | 25 | 25 | 50 |
| csrf | 352 | 25 | 25 | 50 |
| tlsverify | 295 | 25 | 25 | 50 |
| race_condition | 362 | 25 | 25 | 50 |
| fileupload | 434 | 25 | 25 | 50 |
| inputval | 20 | 25 | 25 | 50 |

Frameworks: net/http, gin, chi, echo, fiber, gorilla/mux, beego, gRPC. Tool-agnostic SARIF-based scoring. All 25 categories have minimum 25/25 TP/TN tests. Plus 6 reference apps with 394 classified functions in [vulnerable_apps/go/](vulnerable_apps/go/).

### Rust v0.5.2 -- 1,133 test cases, 25 CWEs, 4 frameworks

| Category | CWE | Vuln | Safe | Total |
|----------|-----|------|------|-------|
| sqli | 89 | 5 | 11 | 16 |
| cmdi | 78 | 14 | 20 | 34 |
| pathtraver | 22 | 14 | 24 | 38 |
| ssrf | 918 | 18 | 20 | 38 |
| xss | 79 | 23 | 23 | 46 |
| memsafety | 119 | 18 | 21 | 39 |
| crypto | 327 | 20 | 21 | 41 |
| weakrand | 330 | 22 | 23 | 45 |
| infodisclosure | 200 | 22 | 25 | 47 |
| deserial | 502 | 23 | 24 | 47 |
| intoverflow | 190 | 23 | 24 | 47 |
| redos | 1333 | 24 | 24 | 48 |
| inputval | 20 | 24 | 25 | 49 |
| hardcodedcreds | 798 | 23 | 25 | 48 |
| race_condition | 362 | 25 | 25 | 50 |
| loginjection | 117 | 25 | 25 | 50 |
| securecookie | 614 | 25 | 25 | 50 |
| redirect | 601 | 25 | 25 | 50 |
| fileupload | 434 | 25 | 25 | 50 |
| tlsverify | 295 | 25 | 25 | 50 |
| authnfailure | 287 | 25 | 25 | 50 |
| csrf | 352 | 25 | 25 | 50 |
| authzfailure | 285 | 25 | 25 | 50 |
| ldapi | 90 | 25 | 25 | 50 |
| nosql | 943 | 25 | 25 | 50 |

Frameworks: actix-web, axum, Rocket, Warp. 1,133 standalone test files in `testcode/`. TP/TN balance: 48/52. Filename-based scoring (post-leakage migration). Reference apps in [vulnerable_apps/rust/](vulnerable_apps/rust/).

### Bash v0.5.3 -- 867 test cases, 20 CWEs

| Category | CWE | Vuln | Safe | Total |
|----------|-----|------|------|-------|
| cmdi | 78 | 11 | 23 | 34 |
| sqli | 89 | 10 | 10 | 20 |
| codeinj | 94 | 14 | 21 | 35 |
| ssrf | 918 | 17 | 20 | 37 |
| auth_bypass | 287 | 25 | 25 | 50 |
| cleartext_tx | 319 | 25 | 25 | 50 |
| dos | 770 | 25 | 25 | 50 |
| hardcoded_creds | 798 | 22 | 25 | 47 |
| infodisclosure | 532 | 22 | 18 | 40 |
| insecure_perms | 732 | 23 | 20 | 43 |
| insecure_temp | 377 | 24 | 24 | 48 |
| loginjection | 117 | 25 | 25 | 50 |
| pathtraver | 22 | 21 | 20 | 41 |
| privilege_escalation | 250 | 25 | 25 | 50 |
| race_condition | 362 | 25 | 25 | 50 |
| rce | 94 | 23 | 22 | 45 |
| ssl_bypass | 295 | 22 | 22 | 44 |
| unquoted | 78 | 18 | 21 | 39 |
| weakrand | 330 | 25 | 25 | 50 |
| weakcrypto | 327 | 22 | 22 | 44 |

867 standalone `benchmark_test_NNNNN.sh` files in `testcode/`. 1-file-1-test architecture (post-leakage migration). TP/TN split: 49/51. Reference apps in [vulnerable_apps/bash/](vulnerable_apps/bash/).

### PHP v0.3.2 -- 1,138 test cases, 25 CWEs

| Category | CWE | Vuln | Safe | Total |
|----------|-----|------|------|-------|
| sqli | 89 | 15 | 15 | 30 |
| xss | 79 | 18 | 18 | 36 |
| cmdi | 78 | 23 | 23 | 46 |
| pathtraver | 22 | 21 | 21 | 42 |
| codeinj | 94 | 24 | 24 | 48 |
| csrf | 352 | 22 | 22 | 44 |
| deserial | 502 | 24 | 24 | 48 |
| extract | 621 | 24 | 24 | 48 |
| fileinclusion | 98 | 23 | 23 | 46 |
| fileupload | 434 | 23 | 23 | 46 |
| hardcodedcreds | 798 | 22 | 22 | 44 |
| headerinj | 113 | 24 | 24 | 48 |
| ldapi | 90 | 24 | 24 | 48 |
| massassign | 915 | 23 | 23 | 46 |
| redirect | 601 | 23 | 23 | 46 |
| securecookie | 614 | 24 | 24 | 48 |
| ssrf | 918 | 23 | 23 | 46 |
| ssti | 1336 | 23 | 23 | 46 |
| typejuggling | 697 | 23 | 23 | 46 |
| unsafereflect | 470 | 24 | 24 | 48 |
| variablevars | 627 | 24 | 24 | 48 |
| weakcipher | 327 | 25 | 25 | 50 |
| weakhash | 328 | 24 | 24 | 48 |
| weakrand | 330 | 23 | 23 | 46 |
| xxe | 611 | 23 | 23 | 46 |

1,138 standalone test files in `testcode/`. Generic `benchmark_test_NNNNN.php` naming with seeded shuffle -- no category leakage in filenames, function names, or comments. Filename-based scoring (same as Go). TP/TN balance: 50/50. 6 PHP-unique CWEs not found in any other benchmark: file inclusion (CWE-98), type juggling (CWE-697), variable extraction (CWE-621), variable variables (CWE-627), unsafe reflection (CWE-470), SSTI (CWE-1336). Plus 4 reference apps with 118 classified snippets in [vulnerable_apps/php/](vulnerable_apps/php/).

### Ruby v0.3.2 -- 1,288 test cases, 27 CWEs, 3 frameworks

| Category | CWE | Vuln | Safe | Total |
|----------|-----|------|------|-------|
| sqli | 89 | 21 | 21 | 42 |
| xss | 79 | 22 | 22 | 44 |
| cmdi | 78 | 23 | 23 | 46 |
| fileupload | 434 | 23 | 23 | 46 |
| csrf | 352 | 23 | 23 | 46 |
| authnfailure | 287 | 25 | 25 | 50 |
| authzfailure | 862 | 25 | 25 | 50 |
| codeinj | 94 | 25 | 25 | 50 |
| deserial | 502 | 23 | 23 | 46 |
| dynmethod | 94 | 25 | 25 | 50 |
| fileinclusion | 98 | 25 | 25 | 50 |
| hardcodedcreds | 798 | 23 | 23 | 46 |
| headerinj | 113 | 24 | 24 | 48 |
| ldapi | 90 | 25 | 25 | 50 |
| loginjection | 117 | 25 | 25 | 50 |
| massassign | 915 | 23 | 23 | 46 |
| pathtraver | 22 | 23 | 23 | 46 |
| redirect | 601 | 23 | 23 | 46 |
| regex | 1333 | 25 | 25 | 50 |
| securecookie | 614 | 23 | 23 | 46 |
| ssrf | 918 | 24 | 24 | 48 |
| ssti | 1336 | 23 | 23 | 46 |
| unsafereflect | 470 | 25 | 25 | 50 |
| weakcipher | 327 | 25 | 25 | 50 |
| weakhash | 328 | 24 | 24 | 48 |
| weakrand | 330 | 24 | 24 | 48 |
| xxe | 611 | 25 | 25 | 50 |

1,288 standalone test files in `testcode/`. Frameworks: Raw Ruby/Rack, Rails 7, Sinatra 3. TP/TN balance: exact 50/50. Filename-based scoring (post-leakage migration). Reference apps in [vulnerable_apps/ruby/](vulnerable_apps/ruby/).

### Adversarial Evasion v0.2.0 -- 123 test cases, 10 categories, cross-language

**This is not a language benchmark. It tests detection techniques that traditional SAST cannot perform.**

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

### Chain Detection v0.2.0 -- 500 test cases, 20 categories, 250 multi-file scenarios

**This is not a vulnerability benchmark. It is a compositional reasoning benchmark.**

While all other benchmarks test whether a tool can detect a single finding, this benchmark tests: **"can your tool correlate multiple low-severity findings across files into a compound exploit path whose severity exceeds any individual component?"** No public benchmark for this capability exists. This is the first.

| Category | Scenarios | Vuln | Safe | Total | Chain Pattern |
|----------|-----------|------|------|-------|---------------|
| unauth_injection | 13 | 13 | 13 | 26 | Auth bypass + injection = unauthenticated attack |
| ssrf_pivot | 13 | 13 | 13 | 26 | SSRF + internal service = network boundary bypass |
| compound_injection | 13 | 13 | 13 | 26 | Injection A enables injection B |
| multi_stage | 13 | 13 | 13 | 26 | Multi-hop attack requiring 2+ vuln types |
| privesc_chain | 13 | 13 | 13 | 26 | Low-priv access + missing authz = privilege escalation |
| xxe_to_file_read | 13 | 13 | 13 | 26 | XXE + file:// URI = server file exfiltration |
| open_redirect_to_phish | 13 | 13 | 13 | 26 | Unvalidated redirect + auth flow = credential theft |
| weak_crypto_to_forge | 13 | 13 | 13 | 26 | Weak crypto + verification = token forgery |
| info_leak_to_account_takeover | 13 | 13 | 13 | 26 | Info disclosure + targeted exploit = account takeover |
| template_injection_to_rce | 13 | 13 | 13 | 26 | SSTI + code execution = RCE |
| idor_data_leak | 12 | 12 | 12 | 24 | Broken object auth + sensitive data = mass exfil |
| race_condition_bypass | 12 | 12 | 12 | 24 | TOCTOU + critical operation = security bypass |
| path_traversal_to_read | 12 | 12 | 12 | 24 | Path traversal + sensitive files = secret leak |
| csrf_to_state_change | 12 | 12 | 12 | 24 | Missing CSRF + state-changing endpoint = unauth actions |
| header_injection_to_cache_poison | 12 | 12 | 12 | 24 | Header injection + caching = persistent XSS |
| mass_assign_to_privesc | 12 | 12 | 12 | 24 | Unfiltered binding + role field = privilege escalation |
| hardcoded_creds_to_access | 12 | 12 | 12 | 24 | Creds in source + auth endpoint = unauthorized access |
| insecure_file_perms_to_tamper | 12 | 12 | 12 | 24 | World-writable file + app trusts it = tampering |
| cors_miscfg_to_data_theft | 12 | 12 | 12 | 24 | Permissive CORS + sensitive API = cross-origin exfil |
| session_fixation_to_hijack | 12 | 12 | 12 | 24 | Session not regenerated + fixable ID = hijack |

Each scenario is a multi-file Flask application with `vuln/` (exploitable chain) and `safe/` (mitigated chain) variants. Safe variants differ by exactly one file. Based on real-world attack patterns including Capital One (2019), CVE-2023-22515, CVE-2021-41773, CVE-2014-3529, CVE-2019-11581, CVE-2022-26138, and PortSwigger cache poisoning research.

See [chains/chain_benchmark.md](chains/chain_benchmark.md) for the full methodology, scenario descriptions, and safe variant design rules.

### Vulnerable Reference Apps -- 26 apps, 884 ground truth entries, 5 languages

Realistic multi-file applications with per-function or per-snippet ground truth. These test cross-file taint propagation, framework-specific sources, MVC architecture, and multi-hop chains that standalone testcode cannot cover. Scored independently from the main benchmarks using line-range matching against `ground_truth.csv` files.

| Language | Apps | Entries | Frameworks |
|----------|------|---------|------------|
| Go | 6 | 394 | gin, echo, chi, fiber, gorilla/mux, beego, gRPC |
| Bash | 5 | 191 | Pure bash (DevOps pipelines, webhook servers) |
| Rust | 8 | 119 | Rocket, Warp, actix-web, axum |
| PHP | 4 | 118 | Laravel, Symfony, WordPress, vanilla PHP |
| Ruby | 3 | 62 | Rack, Rails, Sinatra |

See [vulnerable_apps/README.md](vulnerable_apps/README.md) for scoring methodology and app inventories.

## Combined Scale

| Benchmark | Tests | CWEs/Categories | TP/TN Balance | What It Tests |
|-----------|-------|-----------------|---------------|---------------|
| Go | 1,350 | 25 CWEs | 50/50 | Vulnerability detection |
| PHP | 1,138 | 25 CWEs | 50/50 | Vulnerability detection |
| Bash | 867 | 20 CWEs | 49/51 | Vulnerability detection |
| Ruby | 1,288 | 27 CWEs | 50/50 | Vulnerability detection |
| Rust | 1,133 | 25 CWEs | 48/52 | Vulnerability detection |
| Vulnerable Apps | 884 | Cross-language | Varies | Multi-file app detection |
| Adversarial | 123 | 10 categories | 52/48 | Evasion/concealment detection |
| Chains | 500 | 20 categories | 50/50 | Compound exploit chain detection |
| **Total** | **7,283** | **47 unique CWEs + 30 detection categories** | |

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
python scripts/score_sarif.py results.sarif go/expectedresults-0.5.1.csv
```

**Adversarial evasion benchmark:**
```bash
your-tool scan adversarial/testcode/ --format sarif -o results.sarif
python scripts/score_sarif.py results.sarif adversarial/expectedresults-0.2.0.csv
```

**Chain detection benchmark:**
```bash
your-tool scan chains/scenarios/ --format sarif -o results.sarif
python scripts/score_sarif.py results.sarif chains/expectedresults-0.2.0.csv
```

Root-cause every FN (missed vulnerability) and FP (false alarm). That's where the benchmark earns its value.

## Limitations

Known limitations:

- **Classification accuracy**: Verified to our best ability. Community review welcome. Some edge cases may be debatable.
- **Scale**: OWASP Java has 2,740 tests. We have 7,283 across five languages + reference apps + adversarial + chains. Growing with each release.
- **Self-graded**: We wrote the tests and the answer key. Independent verification is the next milestone.
- **Adversarial coverage**: 123 test cases across 10 categories. Dependency confusion (registry-level) and true AI polymorphic malware (runtime) are not yet covered as they require infrastructure beyond static source files.
- **Chain coverage**: 500 test cases across 20 categories. Python/Flask only. Cross-language chains planned for future releases.

We release this openly because imperfect ground truth that invites correction is more valuable than no ground truth at all.

## Inspiration

- [OWASP Benchmark for Java](https://owasp.org/www-project-benchmark/) -- 2,740 test cases
- [OWASP Benchmark for Python](https://github.com/OWASP-Benchmark/BenchmarkPython) -- 1,230 test cases
- [OWASP Juice Shop](https://owasp.org/www-project-juice-shop/) -- Node.js intentionally vulnerable app

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Apache License 2.0 -- see [LICENSE](LICENSE).
