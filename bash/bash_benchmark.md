# Bash SAST Benchmark

**Created:** 2026-03-19 | **Updated:** 2026-03-22 (v0.3.1 — 356 test cases, 5 apps, 16 CWEs, 49/51 TP/TN)
**Team:** Bash (of 3: Go, Rust, Bash)
**Version:** v0.3.1
**Status:** OWASP rebalancing complete. 16 CWE categories. Awaiting first SAST tool baseline run.

---

## Purpose

Build TP/FP/FN/TN ground truth for Bash shell scripts so any SAST tool's detection accuracy is measurable. Without ground truth, you can't calculate TPR/FPR/Youden's J. You can't track regressions. You can't evolve.

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

## SAST Engine Capabilities (Reference Analysis)

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
+-- apps/                          # 5 annotated applications (191 test cases)
|   +-- pipeline-manager/          # DevOps CI/CD pipeline (10 scripts, 78 test cases)
|   +-- deepflow-webhook/          # HTTP webhook server (8 files, 28 test cases)
|   +-- deepflow-ops/              # Operations suite with SAFE_MODE (7 files, 20 test cases)
|   +-- dataforge/                 # Data pipeline scripts (4 files, 10 test cases)
|   +-- securepipeline/            # Hardened CI/CD pipeline (7 files, 55 TN-only safe cases)
+-- testcode/                      # 16 standalone CWE test files (165 test cases)
+-- expectedresults-0.3.1.csv      # Answer key (356 test cases, OWASP CSV format)
+-- bash_benchmark.py              # Scoring script
+-- BENCHMARK.md                   # This file
+-- CHANGELOG.md                   # Version history
```

---

## Test Case Statistics

| Category | CWE | Total | Vulnerable (TP) | Safe (TN) |
|----------|-----|-------|-----------------|-----------|
| cmdi | 78 | 106 | 53 | 53 |
| sqli | 89 | 42 | 21 | 21 |
| codeinj | 94 | 36 | 18 | 18 |
| ssrf | 918 | 22 | 11 | 11 |
| unquoted | 78 | 20 | 10 | 10 |
| pathtraver | 22 | 18 | 9 | 9 |
| infodisclosure | 200 | 15 | 6 | 9 |
| hardcoded_creds | 798 | 14 | 7 | 7 |
| ssl_bypass | 295 | 13 | 6 | 7 |
| weakcrypto | 327 | 12 | 6 | 6 |
| insecure_perms | 732 | 12 | 5 | 7 |
| rce | 94 | 10 | 5 | 5 |
| weakrand | 330 | 10 | 5 | 5 |
| race_condition | 362 | 10 | 5 | 5 |
| insecure_temp | 377 | 8 | 4 | 4 |
| auth_bypass | 287 | 8 | 4 | 4 |
| **TOTAL** | | **356** | **175** | **181** |

**TP/TN split: 49.2% / 50.8%** — Balanced per-category to match OWASP methodology. A tool that flags everything scores ~50% TPR, not the inflated 68% of v0.3.

**5 applications tested**: Pipeline Manager (DevOps CI/CD), deepflow-webhook (HTTP webhook server), deepflow-ops (operations suite with SAFE_MODE toggle), dataforge (data pipeline backup/deploy/healthcheck), securepipeline (hardened CI/CD with 55 safe-only patterns).

For comparison: OWASP Java = 52/48%, OWASP Python = 37/63%. Our 49/51 is within OWASP's acceptable range.

---

## Scoring

```bash
# After running your SAST tool on this directory:
python3 bash_benchmark.py
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
6. **50/50 TP/TN balance per category** — eliminates the "flag everything" scoring exploit (v0.3.1)

---

## Known Bias Risks

1. **Self-exam problem**: We wrote the test AND grade it. The Java benchmark worked because OWASP wrote it independently.
2. **Pattern awareness**: Test writers had access to the reference SAST tool's source code. Unconscious bias toward detectable patterns is possible despite efforts to avoid it.
3. ~~**TP-heavy split**~~: RESOLVED in v0.3.1. Split is now 49/51, matching OWASP Java methodology.
4. **Bash-specific limitations**: Some vulnerability categories (SSRF, SQL injection) are less common in real bash scripts than in web applications.

**Mitigation**: Document everything. Iterate. When the engine improves, add harder test cases. The benchmark is a living document, not a one-time test.

---

## Detection Coverage Matrix

Maps each benchmark category to expected detection mechanism. See `coverage_cve_gaps.md` in the repo root for detailed gap analysis.

| Category | Expected Detection | Taint Analysis Available? | Expected FN Count | Key Gaps |
|----------|-------------------|--------------------|-------------------|----------|
| cmdi (74) | bash-eval-injection, bash-command-injection-taint, +8 rules | YES (15 flows) | 5-10 | nameref, sed, awk, arg injection, env var as cmd |
| codeinj (24) | bash-source-injection, bash-curl-pipe-bash | Partial | 3-5 | trap, heredoc, eval+$(curl), JSON injection, double eval |
| sqli (27) | **No rule consumes taint data** | YES (19 flows) but UNUSED | 16-21 | GAP-BASH-08: biggest gap |
| pathtraver (16) | **NO bash rule** | YES (21 flows) but UNUSED | 5-9 | GAP-BASH-10, tar traversal, deploy path |
| ssrf (13) | **NO bash rule** | YES (26 flows) but UNUSED | 7-11 | GAP-BASH-09, git clone, backup upload |
| infodisclosure (14) | bash-debug-mode-leak (set -x only) | YES (25 flows) but UNUSED | 3-5 | GAP-BASH-11, incomplete redaction (CWE-532) |
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

Run your SAST tool on this directory, then execute the scoring script.

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
- **2026-03-19**: Phase 3 — Added 7 Tier 2 test cases. ${var@Q} sanitizer, NODE_TLS_REJECT_UNAUTHORIZED, tar member traversal, ORDER BY injection. Quality over quantity — dropped patterns that failed manual source code verification.
- **2026-03-19**: Phase 4 — Polish. Updated all stats to 179.
- **2026-03-19**: Phase 6A — Added deepflow-webhook app (8 files, 28 test cases). CGI eval, JSON field eval, email header injection, double eval templates, SSRF via git clone.
- **2026-03-19**: Phase 6B — Added deepflow-ops app (7 files, 20 test cases). SAFE_MODE toggle pattern, env var as command, git branch injection, mail injection, stdin-fed eval/SQL, cross-service taint.
- **2026-03-19**: Phase 6C — Added dataforge app (4 files, 10 test cases). Incomplete keyword redaction (CWE-532), JSON injection, 5 safe healthcheck functions. Initial automated classifications corrected via manual verification.
- **2026-03-19**: Phase 6D — Final polish. All stats updated to 237 test cases (161 TP / 76 TN). 4 apps, 42 files, 7,716 lines.
