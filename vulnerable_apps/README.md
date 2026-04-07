# Vulnerable Reference Applications

## Overview

26 intentionally vulnerable applications across 5 languages, with per-function/per-snippet ground truth classifications. These complement the standalone `testcode/` benchmarks by testing SAST tools against realistic multi-file projects with cross-function taint flows, multiple frameworks, and mixed vulnerable/safe patterns.

**Why separate from `testcode/`?** The main benchmark uses one-function-per-file with filename-based scoring. Apps use multi-function-per-file with annotation-based or function-level scoring -- different mechanisms that shouldn't be mixed in the same CSV.

## Inventory

| Language | Apps | Ground Truth Entries | Annotation Format | Scoring Ready |
|----------|------|---------------------|-------------------|---------------|
| [Go](go/go_apps_benchmark.md) | 6 | 394 | Per-app `ground_truth.csv` (function-level) | Yes |
| [Rust](rust/rust_apps_benchmark.md) | 8 | 119 | `expectedresults-0.1.0.csv` (annotation-based) | Yes |
| [PHP](php/php_apps_benchmark.md) | 4 | 118 | `expectedresults-0.1.0.csv` (annotation-based) | Yes |
| [Bash](bash/bash_apps_benchmark.md) | 5 | 191 | `expectedresults-0.1.0.csv` (annotation-based) | Yes |
| [Ruby](ruby/ruby_apps_benchmark.md) | 3 | 62 | `expectedresults-0.1.0.csv` (annotation-based) | Yes |
| **Total** | **26** | **884** | | |

## Ground Truth Formats

### Go: Function-Level CSV (per-app)

Each Go app has its own `ground_truth.csv` with function-level classifications:

```csv
# function,file,start_line,end_line,category,vulnerable,CWE
GetUser,internal/handlers/gin_handlers.go,39,55,sqli,true,89
GetUserAlt,internal/handlers/gin_handlers.go,306,319,sqli,false,89
```

Scoring matches SAST findings to function line ranges. A finding within a `vulnerable=true` range is a TP; within a `vulnerable=false` range is a FP.

### Ruby/PHP/Rust/Bash: Annotation-Based CSV (per-language)

Each language directory has a single `expectedresults-0.1.0.csv` using the same format as the main benchmark CSVs:

```csv
# test name,category,real vulnerability,CWE
vb_sqli_login,sqli,true,89
vb_sqli_login_safe,sqli,false,89
```

Test names correspond to inline `vuln-code-snippet` annotations in the source files:

```
// vuln-code-snippet start vb_sqli_login
    $query = "SELECT * FROM users WHERE username = '" . $user . "'"; // vuln-code-snippet vuln-line vb_sqli_login
// vuln-code-snippet end vb_sqli_login
```

Annotation markers: `vuln-line` = vulnerable, `safe-line` = safe. Rust uses `target-line` for all annotations (the CSV determines vulnerability status, not the annotation marker).

## How to Score

### 1. Run your SAST tool on an app directory

```bash
your-tool scan vulnerable_apps/go/multi-api/ --format sarif -o results.sarif
```

### 2. Match findings to ground truth

For each finding the tool produces (file + line number):

| Finding location | Ground truth | Result |
|-----------------|-------------|--------|
| Within a `vulnerable=true` line range | Detected | **True Positive** |
| Within a `vulnerable=false` line range | Detected | **False Positive** |
| Within a `vulnerable=true` line range | Not detected | **False Negative** |
| Within a `vulnerable=false` line range | Not detected | **True Negative** |

### 3. Compute Youden's J

```
TPR = TP / (TP + FN)      Sensitivity (catch rate)
FPR = FP / (FP + TN)      Fall-out (false alarm rate)
Score = TPR - FPR          Youden's J statistic (-100% to +100%)
```

## What Makes Apps Valuable vs. Standalone Tests

| Property | `testcode/` (standalone) | `vulnerable_apps/` (apps) |
|----------|------------------------|--------------------------|
| Functions per file | 1 | Many |
| Cross-file taint | No | Yes (handler -> service -> repo) |
| Framework APIs | Synthetic | Real (gin, beego, gRPC, Rails, Sinatra, actix-web, etc.) |
| Architecture | Flat | MVC layers, middleware, route registration |
| Taint hops | 1-2 | 2-5+ |
| Scoring mechanism | Filename-based | Line-range-based |

## Directory Structure

```
vulnerable_apps/
  README.md                    # This file
  CHANGELOG.md                 # Version history
  dev_roadmap.md               # Planned improvements
  go/                          # 6 apps, 394 entries
    go_apps_benchmark.md       # App inventory and scoring
    multi-api/ground_truth.csv
    calorie-tracker/ground_truth.csv
    go_notifications/ground_truth.csv
    beego_admin/ground_truth.csv
    grpc_users/ground_truth.csv
    cobra_cli_test/            # CLI tool, no attack surface (0 entries)
  rust/                        # 8 apps, 119 entries
    rust_apps_benchmark.md
    expectedresults-0.1.0.csv
  php/                         # 4 apps, 118 entries
    php_apps_benchmark.md
    expectedresults-0.1.0.csv
  bash/                        # 5 apps, 191 entries
    bash_apps_benchmark.md
    expectedresults-0.1.0.csv
  ruby/                        # 3 apps, 62 entries
    ruby_apps_benchmark.md
    expectedresults-0.1.0.csv
```

## Relationship to Main Benchmarks

The main benchmarks (`go/`, `rust/`, `bash/`, `php/`, `ruby/`) use standalone test files with filename-based scoring. App entries were previously mixed into those CSVs but were separated in the v0.5.x migrations to cleanly distinguish synthetic testcode scoring from realistic application scoring.

Apps and testcode share the same CWE categories but are scored independently. A tool's app score and testcode score together give a more complete picture: testcode measures breadth across vulnerability patterns, while apps measure depth across realistic codebases.
