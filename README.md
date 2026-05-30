# BenchProctor

**Ground truth for SAST.** An open, machine-verifiable benchmark corpus for measuring how
accurately a static analysis tool finds real vulnerabilities — and how often it flags safe code.

**[benchproctor.com](https://benchproctor.com)** · [blog](https://blog.benchproctor.com) · Apache-2.0

> ## Release status
>
> The corpus spans nine languages, but we publish each only once it's verified production-ready — we won't ship labels we can't defend.
>
> - **Java — production-ready. Full public launch before the end of June 2026** (Spring + Jakarta EE).
> - **Python — close behind** (Flask / Django / FastAPI).
> - Go, Rust, PHP, Ruby, JavaScript, TypeScript, and Bash follow as each clears the same bar.
>
> Why one language at a time? [Read the release plan →](https://blog.benchproctor.com/java-first-release-plan/)

A SAST tool is only as trustworthy as its accuracy, and accuracy is unmeasurable without ground
truth. BenchProctor gives you labeled corpora — programs marked `vulnerable` or `safe` — so you
can score any tool that emits SARIF 2.1.0 and get a real number: true-positive rate,
false-positive rate, and overall detection accuracy (Youden's J).

## Quick start

```bash
# 1. run your scanner against a corpus, export SARIF 2.1.0
your-tool scan ./corpus --format sarif -o results.sarif

# 2. score against the answer key (standard-library Python, zero dependencies)
python scripts/score_sarif.py results.sarif corpus/expectedresults-*.csv

# 3. read TPR, FPR, and your Youden's J — category-averaged and flat aggregate
```

## Why another benchmark

Existing public SAST benchmarks share three structural weaknesses:

- **Hand-authored and frozen.** A fixed set of human-written cases gets published once and never
  changes, so tools — and the models behind them — overfit to it. A high score stops meaning
  real-world accuracy.
- **The filename leaks the answer.** When a test lives at `sqli/Test01729_true_positive.java`, a
  scanner can score well by matching the path, not by analyzing code.
- **One language, one file, no defenses.** Real findings cross files, services, and languages and
  sit next to sanitizers that almost work. Single-file, single-language suites never exercise that.

BenchProctor is built to remove all three.

## What's in the corpus

| | |
|---|---|
| **Languages** | 9 — Python, Go, Java, JavaScript, TypeScript, PHP, Ruby, Bash, Rust |
| **Frameworks** | 18 — at least two per language, real idioms (DTOs, Pydantic models, ORM calls) |
| **Categories** | 234 |
| **Unique CWEs** | 219 |
| **OWASP Top 10 2025** | 213 / 249 mapped CWEs (85.5%) |
| **Balance** | ~50 / 50 vulnerable / safe |

- **Combinatorial, not hand-written.** Each category is a vulnerability class expressed as a taint
  flow over four axes — where untrusted input enters (**source**), how it travels (**propagator**),
  what would neutralize it (**sanitizer**), and the dangerous call it reaches (**sink**). The corpus
  is assembled by combining those building blocks (42 sources × 40 propagators × 65 sanitizers ×
  58 sinks): a vulnerable case omits an effective sanitizer; its safe twin applies one. Every
  emitted combination is constrained to a realistic flow.
- **Anti-leakage by construction.** Emitted files carry no comments, no CWE tags, no category names,
  and no hints in identifiers. File IDs are shuffled, so a filename reveals nothing about a file's
  category or label. The CSV answer key is the only ground truth.
- **Quarterly rotation.** Each release is generated from a fixed seed that changes *which*
  combinations are emitted — so the actual code differs every quarter — while holding every
  scoring-relevant invariant constant (CWE identity, difficulty distribution, 50/50 balance,
  language/framework coverage). Same seed reproduces the corpus byte-for-byte; a new seed yields
  fresh variants you can't have pre-trained against. Last quarter's score stays comparable.

## Beyond single files — the test shapes

Detecting a bare `eval(input)` is table stakes. The corpus is weighted toward the findings that
separate a real analyzer from a pattern matcher:

- **Cross-file CWE chains.** Instead of one weakness in one file, a chain threads a group of
  smaller CWEs across modules and functions so they compound into a larger compromise — the
  low-severity findings that line up into privilege escalation and root access. Tools that reason
  file-by-file miss the path.
- **Polyglot microservice scenarios.** Taint that crosses language and process boundaries the way
  real systems do: HTTP, gRPC, subprocess, environment variables, and message queues (Kafka,
  RabbitMQ, Redis pub/sub), across 2-, 3-, and 4-service topologies with multiple sources, sinks,
  and paths. A 4-hop scenario forces a tool to track taint across four files and three inter-service
  boundaries to find the vulnerable path.
- **Broken-sanitizer variants.** A sanitizer is present but bypassed — a flawed regex, wrong-context
  escaping, an insufficient length limit. Scanners that trust the mere presence of a sanitizer
  mislabel these as safe.
- **Adversarial & modern-threat cases.** Built to defeat naive static analysis and to cover where
  the threat landscape is actually moving — the Age of AI, not just textbook SQLi: SAST-evasion
  (Unicode homoglyphs, encoding round-trips, dynamic dispatch and reflection, eval/exec wrappers,
  Unicode-normalization filter bypasses), software supply-chain attack shapes (glassworm-style), and
  AI/LLM prompt-injection-era patterns.

## Polyglot coverage

| Language | Frameworks |
|---|---|
| Python | Flask, Django, FastAPI |
| Go | net/http, Gin |
| Java | Spring, Jakarta EE |
| JavaScript | Express, Koa |
| TypeScript | NestJS, Express |
| PHP | Laravel, Symfony |
| Ruby | Rails, Sinatra |
| Bash | standalone |
| Rust | Actix-web, Axum |

Adding a language changes nothing about the categories, so coverage stays uniform across the matrix.

## OWASP Top 10 2025

| Category | Covered / Mapped | |
|---|---|---|
| A01 Broken Access Control | 37 / 40 | 92% |
| A02 Security Misconfiguration | 11 / 16 | 69% |
| A03 Software Supply Chain | 0 / 6 | composition analysis, not code-pattern SAST |
| A04 Cryptographic Failures | 30 / 32 | 94% |
| A05 Injection | 31 / 37 | 83% |
| A06 Insecure Design | 27 / 39 | 69% |
| A07 Authentication Failures | 34 / 36 | 94% |
| A08 Software & Data Integrity | 8 / 14 | 57% |
| A09 Logging & Alerting Failures | 5 / 5 | 100% |
| A10 Exceptional Conditions | 22 / 24 | 92% |

213 of 249 mapped CWEs (85.5%). The remainder is config-level, supply-chain, or runtime-only — not
expressible as a static code pattern.

## Scoring

Every test case carries a ground-truth label (`vulnerable` or `safe`) in a CSV answer key. After a
tool runs, scoring computes a confusion matrix and one subtraction:

```
                detected   ignored
 vulnerable        TP         FN
 safe              FP         TN

 TPR  = TP / (TP + FN)     detection rate
 FPR  = FP / (FP + TN)     false-alarm rate
 J    = TPR - FPR          Youden's J  (the score)
```

| Score | Meaning |
|------:|---------|
| +100% | Perfect — catches everything, zero false alarms |
| 0% | No better than guessing (where a flag-everything tool lands on a 50/50 corpus) |
| -100% | Inverted — flags safe code, misses real bugs |

Scores are reported two ways: **category-averaged** (each category weighted equally so large
categories can't dominate — the headline number) and **flat aggregate**. Any tool that emits SARIF
2.1.0 can be scored; the scorer is a single standard-library Python file with no dependencies.

## Machine-verifiable ground truth

Every corpus ships a **proof manifest** — one record per file naming its exact source, propagator,
sanitizer, sink, difficulty, the sink's line number, and a SHA-256 of the file — so any label can be
independently audited from metadata alone. A bundled **self-test SARIF** scores a perfect Youden's J
against the answer key, proving the labels and the scorer agree before the benchmark can mislead you.

## Releases

Corpora are versioned and released quarterly. The scorer in `scripts/score_sarif.py` is
standard-library Python only — clone, point it at a corpus and your SARIF, and read your number.

## License

Apache License 2.0 — see [LICENSE](LICENSE). Created and maintained by the author of BenchProctor.
