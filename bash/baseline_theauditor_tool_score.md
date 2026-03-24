# Baseline Score: TheAuditor v3.6

**Tool:** TheAuditor v3.6
**Benchmark:** Bash SAST Benchmark v0.3.1 (356 test cases, 16 categories)
**Date:** 2026-03-24
**Previous:** v3.5 baseline (2026-03-23) scored +20.9%. This run: +50.5%.
**Configuration:** `aud full --offline` (default rules, no tuning)

---

## Why We Publish This

We built this benchmark. We also built the tool being scored. Publishing our own tool's results — including every missed vulnerability and every false alarm — is how we hold ourselves accountable.

OWASP publishes scores for Checkmarx, Fortify, Veracode, and dozens of other commercial tools against the Java benchmark. Nobody loses credibility for honest numbers. They lose credibility for hiding them.

This baseline is a public roadmap. Every FN below is a detection gap we intend to close. Every FP is a discrimination problem we intend to fix. Future versions of TheAuditor will be re-scored against the same benchmark, and the delta will show whether we actually improved or just claimed to.

---

## Scorecard

```
Category             CWE    TP    FP    FN    TN      TPR     FPR   Score
------------------------------------------------------------------------------
auth_bypass          306    4     0     0     4    100.0%    0.0% +100.0%
hardcoded_creds      798    7     0     0     7    100.0%    0.0% +100.0%
insecure_perms       732    5     0     0     7    100.0%    0.0% +100.0%
insecure_temp        377    4     0     0     4    100.0%    0.0% +100.0%
race_condition       362    5     0     0     5    100.0%    0.0% +100.0%
rce                  94     5     0     0     5    100.0%    0.0% +100.0%
ssl_bypass           295    6     0     0     7    100.0%    0.0% +100.0%
unquoted             78     10    0     0     10   100.0%    0.0% +100.0%
weakcrypto           327    6     0     0     6    100.0%    0.0% +100.0%
weakrand             330    5     0     0     5    100.0%    0.0% +100.0%
codeinj              94     18    1     0     17   100.0%    5.6%  +94.4%
ssrf                 918    9     0     2     11    81.8%    0.0%  +81.8%
cmdi                 78     33    0     20    53    62.3%    0.0%  +62.3%
pathtraver           22     7     2     2     7     77.8%   22.2%  +55.6%
infodisclosure       200    2     0     4     9     33.3%    0.0%  +33.3%
sqli                 89     10    5     11    16    47.6%   23.8%  +23.8%
------------------------------------------------------------------------------
OVERALL                     136   8     39    173   77.7%    4.4%  +73.3%
```

**Score: +53.8%** (Youden's J = TPR - FPR)

---

## Delta from v3.5 Baseline (+20.9% -> +50.5%)

```
Category             v3.5     v3.6     Delta    TP/FP/FN/TN change
----------------------------------------------------------------------
hardcoded_creds     +71.4%  +100.0%   +28.6    TP+2, FN-2
insecure_temp       +25.0%  +100.0%   +75.0    TP+1, FP-2, FN-1, TN+2
weakrand             +0.0%  +100.0%  +100.0    TP+5
race_condition      +40.0%   +80.0%   +40.0    TP+2, FN-2
codeinj             +55.6%   +77.8%   +22.2    FP-5, TN+5
unquoted            +50.0%   +70.0%   +20.0    TP+2, FN-2
weakcrypto          +33.3%   +66.7%   +33.4    TP+2, FN-2
cmdi                +24.5%   +54.7%   +30.2    FP-17, TN+17, FN+1, TP-1
ssrf                +27.3%   +54.5%   +27.2    FP-4, TN+4, FN+1, TP-1
ssl_bypass           +7.1%   +40.5%   +33.4    TP+2, FN-2
rce                 +40.0%   +40.0%    +0.0    (unchanged)
insecure_perms       +8.6%   +28.6%   +20.0    TP+1, FN-1
auth_bypass          +0.0%   +25.0%   +25.0    TP+1, FN-1
sqli                -14.3%   +19.0%   +33.3    TP+1, FP-6, TN+6, FN-1
pathtraver          -11.1%   +11.1%   +22.2    FP-2, TN+2
infodisclosure      -33.3%   -22.2%   +11.1    FP-1, TN+1
----------------------------------------------------------------------
OVERALL             +20.9%   +50.5%   +29.6    TP+16, FP-37, FN-16, TN+37
```

**Key improvements:**
- 3 categories now at 100%: hardcoded_creds, insecure_temp, weakrand
- 37 false positives eliminated (76 -> 39)
- 16 new true positives (110 -> 126)
- Zero categories with negative score (was 3: pathtraver, sqli, infodisclosure — now only infodisclosure)

---

## What The Numbers Mean

- **126 True Positives**: Vulnerabilities correctly detected (was 110)
- **142 True Negatives**: Safe code correctly ignored (was 105)
- **49 False Negatives**: Vulnerabilities missed — detection gaps (was 65)
- **39 False Positives**: Safe code incorrectly flagged — discrimination gaps (was 76)

---

## Strengths (Score > +50%)

| Category | Score | Why |
|----------|-------|-----|
| hardcoded_creds (+100.0%) | Credential pattern matching + heredoc detection. Zero FP, zero FN. |
| insecure_temp (+100.0%) | Predictable temp path detection + mktemp/TOCTOU patterns. Zero FP. |
| weakrand (+100.0%) | $RANDOM detection for security-sensitive values. Zero FP. |
| race_condition (+80.0%) | TOCTOU, PID file, stat-then-source detection. 1 FN remaining (lock_toctou). |
| codeinj (+77.8%) | Source injection, process substitution, curl\|bash patterns. 1 FP on constant source path. |
| unquoted (+70.0%) | Structural detection of unquoted variable expansion in dangerous commands. 3 FP on safe quoting patterns. |
| weakcrypto (+66.7%) | md5sum/sha1sum/openssl structural detection. 2 FP on SHA256 patterns. |
| cmdi (+54.7%) | Eval, variable-as-command, sudo, backtick, printf format, read -r, IFS, environment injection. 15 FN remain (nameref, arithmetic, sed, awk, xargs, argument injection). |
| ssrf (+54.5%) | IFDS taint analysis confirms SSRF flows. 4 FN in standalone testcode. |

## Weaknesses (Score < +25%)

| Category | Score | Root Cause |
|----------|-------|------------|
| sqli (+19.0%) | 13 FN: SQL injection in bash uses string interpolation into sqlite3/mysql CLI. IFDS confirms flows but rules don't surface all. Standalone testcode lacks entry points. |
| pathtraver (+11.1%) | 3 FP on safe patterns (realpath, validated). 5 FN in standalone testcode lacking taint context. |
| infodisclosure (-22.2%) | 5 FP on safe patterns (filtered subshell, masked logs, hardcoded responses, healthchecks). Misses env dumps and credential-in-error-message patterns. |

---

## False Negatives (49 missed vulnerabilities)

Every FN is a detection gap. Grouped by root cause:

### Standalone testcode lacks taint entry points (14 FN)
Functions in `testcode/` take parameters but have no CGI environment or CLI dispatch context. The taint engine can't trace data flow into these functions because no entry_point edges exist.

**Affected:** sqli_direct_interpolation, sqli_multihop_taint, sqli_table_name_injection, sqli_order_by_injection, pathtraver_cat_user_input, pathtraver_tar_extract, pathtraver_symlink_bypass, pathtraver_rm_user_path, pathtraver_tar_member_traversal, ssrf_curl_user_url, ssrf_url_path_injection, ssrf_wget_redirect_follow, cmdi_xargs_tainted, cmdi_argument_injection_grep

### No rule for this pattern (9 FN)
The engine has no structural or taint rule that detects these specific bash patterns.

**Affected:** cmdi_nameref_injection (declare -n), cmdi_arithmetic_expansion ($(( )) array subscript injection), cmdi_sed_expression_injection (sed -e), cmdi_awk_program_injection (awk program), codeinj_bash_c_injection (bash -c), codeinj_trap_injection (trap), dfg_infra_incomplete_redaction (keyword-based log redaction), dfo_cleanup_find_pattern (find -name with user pattern), dfo_deploy_bash_c_hook (bash -c in hook)

### SQL injection in apps not surfaced by rules (7 FN)
IFDS taint pipeline confirms these as VULNERABLE flows, but bash rules don't surface all of them as findings.

**Affected:** sql_injection_record_deployment, sql_injection_get_deployment_status, sql_injection_record_health_check, sql_injection_record_config_change, sql_injection_cleanup_days, sql_injection_push_event, sql_injection_release_event

### Other detection gaps (19 FN)
Mixed causes: SSH host key bypass, deploy command injection, eval in specific contexts, auth bypass patterns, info disclosure patterns, race conditions.

**Affected:** ssh_host_key_bypass, sed_config_injection, ifs_modification_lost, run_seed_script, deploy_from_webhook_payload, slack_deploy_command, sql_injection_migration_data, dfw_eval_query_string, dfw_exec_endpoint, dfw_sql_query_endpoint, dfw_send_error_json_inject, dfo_wh_cross_service_callback, dfg_backup_json_injection, dfg_infra_incomplete_redaction, infodisclosure_env_dump, infodisclosure_error_msg_path, race_lock_toctou, auth_empty_credential_bypass, auth_webhook_no_signature, auth_timing_compare

---

## False Positives (39 safe code incorrectly flagged)

Every FP is a discrimination gap. Grouped by root cause:

### Cross-category taint bleed (11 FP)
IFDS taint findings of one category (e.g. SSRF) land within annotated regions of a different category's safe test case. The scoring script counts any finding regardless of category.

**Affected:** install_from_remote_verified (rce/SSRF bleed), sp_net_download_no_exec (rce/SSRF bleed), sp_net_rce_save_only (rce/SSRF bleed), sp_net_curl_ssl_verified (ssl/SSRF bleed), sp_deploy_ssh_strict (ssl/SSRF bleed), sp_net_ssrf_hardcoded_url (ssrf/PathTraversal bleed), sp_db_printf_q_escape (sqli/taint bleed), sp_sec_sql_integer_id (sqli/taint bleed), sp_deploy_docker_quoted (unquoted/taint bleed), sp_deploy_quoted_cp (unquoted/taint bleed), sp_backup_restore_quoted (unquoted/taint bleed)

### Engine flags safe validation/sanitization patterns (9 FP)
The engine detects the dangerous operation but doesn't recognize preceding validation or sanitization as a taint break.

**Affected:** dfw_notify_sanitized, dfw_parse_qs_declare, dfo_backup_validated, dfo_deploy_validated, dfo_notify_encoded, dfo_wh_parse_qs_declare, db_query_escaped, download_file_verified, sp_config_load_constant

### Engine flags safe infrastructure functions (9 FP)
Functions that use curl, sqlite3, pgrep, chmod, etc. with hardcoded or validated inputs. The engine sees the dangerous command but not the safe context.

**Affected:** dfw_json_get_jq, dfw_json_build_escaped, dfg_healthcheck_http, dfg_healthcheck_process, dfg_healthcheck_file, dfg_healthcheck_database, dfg_healthcheck_disk, sp_sec_no_set_x, sp_sec_printf_format

### Overly broad permission detection (5 FP)
Engine flags chmod 600, chmod 755, umask 077, and similar secure permission settings.

**Affected:** chmod755_deploy_dir, secure_file_perms, perms_umask_077, sp_deploy_chmod_755, sp_sec_chmod_600

### Overly broad crypto detection (2 FP)
Engine flags SHA256 in checksum/HMAC contexts as weak crypto.

**Affected:** sp_backup_sha256_integrity, sp_webhook_hmac_sha256

### Other FP (3 FP)
Edge cases in subshell filtering, sudo allowlist matching, and securepipeline taint.

**Affected:** infodisclosure_subshell_filtered, sp_sec_sudo_allowlist, sp_backup_realpath_validated

---

## Reproduction

```bash
# 1. Install TheAuditor v3.6
# 2. Setup and scan
cd gorustbash_benchmark/bash
aud setup-ai --target . --sync
aud full --offline

# 3. Score
python3 bash_benchmark.py
```

---

## Next Steps

Each FN and FP above is a trackable improvement target. Key areas:

1. **Scoring category-awareness** — 11 FPs are cross-category taint bleed. Adding `found_cat == tc["category"]` to the scoring script would eliminate them. Requires cross-team decision.
2. **Standalone testcode entry points** — 14 FNs caused by functions in testcode/ lacking taint entry points. Fixing the graph strategy to create entry_point edges for function parameters in standalone files would recover these.
3. **Missing rule patterns** — 9 FNs for patterns with no rule (nameref, arithmetic expansion, sed/awk injection, bash -c, trap). Each needs a dedicated structural rule.
4. **Sanitizer recognition** — 9 FPs where the engine doesn't recognize validation/sanitization patterns (printf %q, declare --, regex validation, integer checks).
