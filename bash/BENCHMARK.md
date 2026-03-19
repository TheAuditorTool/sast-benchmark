# Bash SAST Benchmark

**Created:** 2026-03-19 | **Updated:** 2026-03-19 (Phase 4 complete — 179 test cases)
**Team:** Bash (of 3: Go, Rust, Bash)
**Status:** Ground truth + engine analysis complete. Awaiting first `aud full --offline` run.

---

## Purpose

Build TP/FP/FN/TN ground truth for Bash shell scripts so TheAuditor's evolution is measurable. Without ground truth, we can't calculate TPR/FPR/Youden's J. We can't track regressions. We can't evolve.

The Java OWASP benchmark (2,740 test cases, independently written) forced 3 major engine overhauls because it exposed blind spots nobody anticipated. This benchmark attempts the same for Bash, with the honest caveat that **we're writing our own exam** — the bias risk is real and documented.

---

## App Under Test: DevOps CI/CD Pipeline Manager

The original bash project (`C:\Users\santa\Desktop\bash\`) is a production-style DevOps pipeline orchestrator with 10 shell scripts across 3 layers:

| File | Purpose | Lines | Test Cases |
|------|---------|-------|------------|
| `pipeline.sh` | CLI router (deploy/rollback/status/etc) | 320 | 0 (orchestrator only) |
| `lib/config.sh` | Config management, env loading, plugin system | 307 | 12 |
| `lib/database.sh` | SQLite deployment tracking, locks, migrations | 383 | 9 |
| `lib/deploy.sh` | SSH/SCP deployment, Docker, K8s, hooks | 404 | 7 |
| `lib/network.sh` | HTTP client, Slack, webhooks, DNS | 374 | 6 |
| `lib/security.sh` | Auth, tokens, encryption, permissions | 448 | 11 |
| `lib/utils.sh` | Logging, string ops, file ops, hashing | 306 | 8 |
| `scripts/backup.sh` | Backup/restore, remote upload, encryption | 449 | 10 |
| `scripts/migrate.sh` | DB schema migrations, seed data | 346 | 7 |
| `scripts/webhook.sh` | CGI-style webhook handler (GitHub/GitLab/Slack) | 540 | 8 |

**Vulnerability profile** (from existing `Desktop/bash/.pf/repo_index.db`):
- 107 pattern findings (rule detections)
- 106 VULNERABLE taint flows (IFDS-confirmed)
- 109 SANITIZED flows (properly defended)
- 254 REACHABLE flows (unsafe but unconfirmed)
- 207 taint sinks, 364 taint sources across all files

---

## TheAuditor Bash Engine Capabilities

### Pipeline Architecture (from correctness_sop.md)

```
Source Code -> AST Extractor (bash_impl.py, tree-sitter)
           -> Post-Processor (indexer/extractors/bash.py)
           -> repo_index.db (bash_symbols, bash_commands, bash_assignments, etc.)
           -> Graph Strategies (bash_pipes.py, bash_cli.py, bash_http_client.py)
           -> graphs.db (entry_point, exit_point, call, pipe_flow edges)
           -> Taint Discovery (discoverers/bash.py + patterns/bash.py)
           -> Rules (injection_analyze, dangerous_patterns_analyze, quoting_analyze)
           -> pattern_findings + resolved_flow_audit
```

### DB Tables Created for Bash

| Table | Purpose | Key Columns |
|-------|---------|-------------|
| `bash_symbols` | Function definitions | name, line, style (posix/bash) |
| `bash_commands` | Command invocations | command_name, containing_function, wrapped_command |
| `bash_function_call_args` | Arguments with quote tracking | argument_expr, is_quoted, has_expansion |
| `bash_assignments` | Variable assignments + data flow | target_var, source_expr, scope |
| `bash_pipes` | Pipe chains | pipeline_id, position, command_text |
| `bash_subshells` | Command substitution | syntax (dollar_paren/backtick), capture_target |
| `bash_sources` | source/dot includes | sourced_path, has_variable_expansion |
| `bash_redirections` | I/O redirects | direction, target, fd_number |
| `bash_control_flows` | if/case/for/while | type, condition, case_value |
| `bash_cfg_blocks/edges` | Control flow graph | block_type, condition_expr |
| `bash_assignment_sources` | DFG: var->var flow | source_var_name -> assignment_target |

### Rules That Fire (27 distinct, verified from DB)

| Rule | Category | What It Detects |
|------|----------|-----------------|
| `bash-eval-injection` | cmdi | eval/bash -c with variable expansion |
| `bash-exec-injection` | cmdi | exec with variable |
| `bash-command-injection-taint` | cmdi | IFDS-confirmed taint->command flows |
| `bash-source-injection` | codeinj | source/dot with variable path |
| `bash-variable-as-command` | cmdi | $cmd arg1 arg2 |
| `bash-backtick-injection` | cmdi | backtick with variable |
| `bash-indirect-expansion` | cmdi | ${!var} |
| `bash-sudo-variable` | cmdi | sudo $cmd |
| `bash-printf-format-injection` | cmdi | printf "$format" |
| `bash-read-without-r` | cmdi | read without -r flag |
| `bash-ifs-modified` | cmdi | IFS modification without restore |
| `bash-environment-injection` | cmdi | LD_PRELOAD, LD_LIBRARY_PATH |
| `bash-path-modification` | cmdi | PATH=./bin:$PATH |
| `bash-unquoted-expansion` | unquoted | Unquoted vars in commands |
| `bash-unquoted-dangerous` | unquoted | Unquoted + dangerous cmd (rm, cp) |
| `bash-hardcoded-credential` | hardcoded_creds | PASSWORD/TOKEN/SECRET vars |
| `secret-hardcoded-assignment` | hardcoded_creds | Language-agnostic secret rule |
| `bash-curl-pipe-bash` | rce | curl/wget piped to bash/sh |
| `bash-ssl-bypass` | ssl_bypass | curl -k, wget --no-check-certificate, SSH StrictHostKeyChecking |
| `bash-debug-mode-leak` | infodisclosure | set -x / set -o xtrace |
| `bash-chmod-777` | insecure_perms | chmod 777 |
| `bash-chmod-666` | insecure_perms | chmod 666 |
| `bash-weak-crypto` | weakcrypto | md5sum, sha1sum |
| `bash-unsafe-temp` | insecure_temp | Predictable /tmp paths |
| `bash-relative-sensitive-cmd` | (noise) | Relative path for security commands |
| `bash-missing-set-e` | (noise) | Missing errexit |
| `bash-missing-set-u` | (noise) | Missing nounset |

### Known Engine Gaps (Expected FN in Benchmark)

These patterns have NO rule. The benchmark includes them deliberately as FN-generators — they're the benchmark's primary VALUE.

| Test Case | Gap | Pipeline Stage |
|-----------|-----|---------------|
| `cmdi_nameref_injection` | `declare -n` not in injection rules | Rules (injection_analyze.py) |
| `cmdi_sed_expression_injection` | sed pattern injection not detected | Rules (no sed-specific rule) |
| `cmdi_awk_program_injection` | awk program injection not detected | Rules (no awk-specific rule) |
| `codeinj_trap_injection` | trap with variable string not detected | Rules (no trap-specific rule) |
| `codeinj_heredoc_expansion` | Heredoc content not analyzed | AST Extractor + Rules |
| `perms_umask_000` | umask not in permission rules | Rules (dangerous_patterns_analyze.py) |
| `perms_chmod_arwx` | Symbolic chmod (a+rwx) not matched | Rules (only numeric 777/666) |
| `ssl_git_no_verify` | GIT_SSL_NO_VERIFY env var not detected | Rules (ssl rule only checks curl/wget/ssh flags) |
| `sqli_multihop_taint` | 3-function taint hop may exceed IFDS | Taint (IFDS depth) |
| `sqli_mysql_cli` | mysql -e in adversarial file lacks taint source | Taint (no entry_point for function param in standalone file) |
| `sqli_table_name_injection` | Identifier injection (table name) not distinguished | Rules (no identifier injection concept) |

---

## Benchmark Structure

```
gorustbash_benchmark/bash/
+-- original/              # Exact copy of Desktop/bash/ with annotations added
|   +-- pipeline.sh        # CLI router (no test cases — orchestrator only)
|   +-- lib/
|   |   +-- config.sh      # 12 test cases (hardcoded creds, eval, source injection, IFS)
|   |   +-- database.sh    # 9 test cases (SQL injection patterns + safe variants)
|   |   +-- deploy.sh      # 7 test cases (SSH, docker, chmod, custom hooks)
|   |   +-- network.sh     # 6 test cases (SSL bypass, curl|bash, safe download)
|   |   +-- security.sh    # 11 test cases (read -r, printf, sudo, permissions)
|   |   +-- utils.sh       # 8 test cases (temp files, unquoted, weak crypto)
|   +-- scripts/
|       +-- backup.sh      # 10 test cases (path traversal, SSRF, eval, unquoted)
|       +-- migrate.sh     # 7 test cases (source injection, SQL injection)
|       +-- webhook.sh     # 8 test cases (eval, SQL injection, SSRF, source)
+-- adversarial/           # 13 NEW test case files, purpose-built for benchmark
|   +-- cmdi_tests.sh      # 12 test cases (eval, nameref, arithmetic, sed, awk)
|   +-- codeinj_tests.sh   # 10 test cases (source, bash -c, trap, heredoc)
|   +-- sqli_tests.sh      # 6 test cases (multi-hop, table injection, mysql)
|   +-- pathtraver_tests.sh # 6 test cases (symlink bypass, realpath check)
|   +-- ssrf_tests.sh      # 5 test cases (allowlist, redirect follow)
|   +-- infodisclosure_tests.sh   # 5 test cases (set -x, env dump, masking)
|   +-- hardcoded_creds_tests.sh  # 6 test cases (literal, CLI args, vault)
|   +-- weakcrypto_tests.sh       # 5 test cases (md5, des, sha1 vs sha256, aes)
|   +-- insecure_temp_tests.sh    # 4 test cases (predictable vs mktemp)
|   +-- insecure_perms_tests.sh   # 5 test cases (777, umask, a+rwx)
|   +-- ssl_bypass_tests.sh       # 5 test cases (--insecure, GIT_SSL_NO_VERIFY)
|   +-- unquoted_tests.sh         # 6 test cases (word splitting, quoting)
|   +-- rce_tests.sh              # 5 test cases (curl|bash, process substitution)
+-- bash_ground_truth.yml  # THE answer key (179 test cases)
+-- bash_benchmark.py      # Scoring script
+-- BENCHMARK.md           # This file
```

---

## Test Case Statistics

| Category | CWE | Total | Vulnerable (TP) | Safe (TN) |
|----------|-----|-------|-----------------|-----------|
| cmdi | 78 | 43 | 31 | 12 |
| codeinj | 94 | 19 | 13 | 6 |
| sqli | 89 | 24 | 19 | 5 |
| pathtraver | 22 | 9 | 6 | 3 |
| ssrf | 918 | 8 | 6 | 2 |
| infodisclosure | 200 | 7 | 4 | 3 |
| hardcoded_creds | 798 | 11 | 7 | 4 |
| weakcrypto | 327 | 9 | 6 | 3 |
| insecure_temp | 377 | 8 | 4 | 4 |
| insecure_perms | 732 | 9 | 5 | 4 |
| ssl_bypass | 295 | 11 | 6 | 5 |
| unquoted | 78v | 13 | 10 | 3 |
| rce | 94v | 8 | 5 | 3 |
| **TOTAL** | | **179** | **122** | **57** |

**TP/TN split: 68.2% / 31.8%** — Skewed toward TP because original files are intentionally vulnerability-heavy code. Adversarial files have better balance (~50/50). Future iterations should add more safe (TN) variants.

**Expected outcomes**: 14 test cases tagged [EXPECTED_FN] (engine has no rule). 3 tagged [EXPECTED_FP] (engine may incorrectly flag safe code).

For comparison: OWASP Java = 52/48%, OWASP Python = 37/63%.

---

## Scoring

```bash
# After running aud full --offline on this directory:
/mnt/c/Users/santa/Desktop/TheAuditorV2/.venv/Scripts/python.exe bash_benchmark.py
```

Score formula: `Score = TPR - FPR` (Youden's J statistic)
- **TPR** = TP / (TP + FN) — sensitivity, recall
- **FPR** = FP / (FP + TN) — false alarm rate
- **+100%** = perfect detection, zero false positives
- **0%** = random guessing
- **-100%** = perfectly wrong

---

## Adversarial Design Philosophy

1. **Ground truth written BEFORE code** — prevents unconscious tailoring
2. **Every TP has a TN sibling** — discrimination is the test, not detection alone
3. **Includes patterns likely to be missed** — nameref injection, arithmetic expansion, sed command injection, heredoc expansion, multi-hop taint
4. **Includes safe patterns that LOOK dangerous** — eval on constants, validated variables in command position, quoted expansions
5. **Inspired by real-world CVE patterns** — not synthetic toy examples

---

## Known Bias Risks

1. **Self-exam problem**: We wrote the test AND grade it. The Java benchmark worked because OWASP wrote it independently.
2. **Pattern awareness**: The test writer has read TheAuditor source code. Unconscious bias toward detectable patterns is inevitable despite efforts to avoid it.
3. **TP-heavy split**: 68.2% TP means the benchmark is easier to score on than a balanced one. A tool that flags everything gets 100% TPR but also 100% FPR.
4. **Bash-specific limitations**: Some vulnerability categories (SSRF, SQL injection) are less common in real bash scripts than in web applications.

**Mitigation**: Document everything. Iterate. When the engine improves, add harder test cases. The benchmark is a living document, not a one-time test.

---

## Detection Coverage Matrix

Maps each benchmark category to expected detection mechanism. See `coverage_cve_gaps.md` in TheAuditorV2 root for full gap analysis.

| Category | Expected Detection | Track A Available? | Expected FN Count | Key Gaps |
|----------|-------------------|--------------------|-------------------|----------|
| cmdi (43) | bash-eval-injection, bash-command-injection-taint, +8 rules | YES (15 flows) | 3-5 | nameref, sed, awk, arg injection |
| codeinj (19) | bash-source-injection, bash-curl-pipe-bash | Partial | 2-4 | trap, heredoc, eval+$(curl) |
| sqli (24) | **NO bash rule consumes Track A** | YES (19 flows) but UNUSED | 15-19 | GAP-BASH-08: biggest gap |
| pathtraver (9) | **NO bash rule** | YES (21 flows) but UNUSED | 4-6 | GAP-BASH-10, tar traversal |
| ssrf (8) | **NO bash rule** | YES (26 flows) but UNUSED | 4-6 | GAP-BASH-09 |
| infodisclosure (7) | bash-debug-mode-leak (set -x only) | YES (25 flows) but UNUSED | 2-4 | GAP-BASH-11 |
| hardcoded_creds (11) | bash-hardcoded-credential + secret-hardcoded-assignment | N/A (structural) | 1-2 | heredoc creds likely FN |
| weakcrypto (9) | bash-weak-crypto (md5sum/sha1sum) | N/A (structural) | 2-3 | openssl not checked |
| insecure_temp (8) | bash-unsafe-temp | N/A (structural) | 1-2 | timestamp, TOCTOU |
| insecure_perms (9) | bash-chmod-777, bash-chmod-666 | N/A (structural) | 2-3 | umask, symbolic chmod |
| ssl_bypass (11) | bash-ssl-bypass, bash-ssh-hostkey-bypass | N/A (structural) | 2-3 | env var bypass, NODE_TLS |
| unquoted (13) | bash-unquoted-expansion, bash-unquoted-dangerous | N/A (structural) | 0-2 | Strong coverage |
| rce (8) | bash-curl-pipe-bash | N/A (structural) | 0-1 | Strong coverage |

**Critical finding**: The engine's IFDS taint pipeline confirms 91 non-Command-Injection flows as VULNERABLE (SQL Injection: 19, SSRF: 26, Path Traversal: 21, Info Disclosure: 25) but **no bash rule reads them**. This is the single biggest coverage gap — fixing it requires adding `get_confirmed_flows()` calls for each vulnerability type, following the existing Command Injection pattern in injection_analyze.py.

---

## Current Scorecard

**Status: AWAITING FIRST RUN**

Run `aud full --offline` on this directory, then execute the scoring script.

```
Category             CWE    TP    FP    FN    TN      TPR     FPR   Score
------------------------------------------------------------------------------
(awaiting first run)
------------------------------------------------------------------------------
OVERALL                     ?     ?     ?     ?      ?.?%    ?.?%   +?.?%
```

**Expected baseline prediction** (before running): Given 17 gaps, structural rules should cover cmdi/hardcoded_creds/weakcrypto/insecure_perms/ssl_bypass/unquoted/rce well. sqli/pathtraver/ssrf will score ~0% (no rule reads IFDS flows). Overall score likely 30-50%.

---

## Changelog

- **2026-03-19**: Initial benchmark creation. 158 test cases across 13 categories.
- **2026-03-19**: Iteration 2 — due diligence. Engine analysis, rule inventory, RULE_MAP fixed with 27 real rule names.
- **2026-03-19**: Iteration 3 — full rules audit. Detection coverage matrix. 17 gaps in coverage_cve_gaps.md.
- **2026-03-19**: Phase 1 — Fixed 1 annotation placement (heredoc marker). Verified 3 audit false alarms (arithmetic expansion IS vulnerable, other markers were already correct).
- **2026-03-19**: Phase 2 — Added 14 Tier 1 test cases. find -exec injection, eval+$(curl), source+<(), argument injection, multi-step eval chain, printf %q sanitizer, TOCTOU race, credentials in heredoc. Each verified exploitable.
- **2026-03-19**: Phase 3 — Added 7 Tier 2 test cases. ${var@Q} sanitizer, NODE_TLS_REJECT_UNAUTHORIZED, tar member traversal, ORDER BY injection. Quality over quantity — dropped patterns that failed Prime Directive verification.
- **2026-03-19**: Phase 4 — Polish. Updated all stats (179 test cases, 122 TP, 57 TN, 68.2%/31.8%). 14 expected FN, 3 expected FP documented.
