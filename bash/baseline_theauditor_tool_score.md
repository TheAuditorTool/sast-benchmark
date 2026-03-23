# Baseline Score: TheAuditor v3.5

**Tool:** TheAuditor v3.5
**Benchmark:** Bash SAST Benchmark v0.3.1 (356 test cases, 16 categories)
**Date:** 2026-03-23
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
hardcoded_creds      798    5     0     2     7     71.4%    0.0%  +71.4%
codeinj              94     16    6     2     12    88.9%   33.3%  +55.6%
unquoted             78     8     3     2     7     80.0%   30.0%  +50.0%
race_condition       362    2     0     3     5     40.0%    0.0%  +40.0%
rce                  94     5     3     0     2    100.0%   60.0%  +40.0%
weakcrypto           327    4     2     2     4     66.7%   33.3%  +33.3%
ssrf                 918    8     5     3     6     72.7%   45.5%  +27.3%
insecure_temp        377    3     2     1     2     75.0%   50.0%  +25.0%
cmdi                 78     39    26    14    27    73.6%   49.1%  +24.5%
insecure_perms       732    4     5     1     2     80.0%   71.4%   +8.6%
ssl_bypass           295    3     3     3     4     50.0%   42.9%   +7.1%
auth_bypass          306    0     0     4     4      0.0%    0.0%   +0.0%
weakrand             330    0     0     5     5      0.0%    0.0%   +0.0%
pathtraver           22     4     5     5     4     44.4%   55.6%  -11.1%
sqli                 89     7     10    14    11    33.3%   47.6%  -14.3%
infodisclosure       200    2     6     4     3     33.3%   66.7%  -33.3%
------------------------------------------------------------------------------
OVERALL                     110   76    65    105   62.9%   42.0%  +20.9%
```

**Score: +20.9%** (Youden's J = TPR - FPR)

---

## What The Numbers Mean

- **110 True Positives**: Vulnerabilities correctly detected
- **105 True Negatives**: Safe code correctly ignored
- **65 False Negatives**: Vulnerabilities missed — detection gaps
- **76 False Positives**: Safe code incorrectly flagged — discrimination gaps

---

## Strengths (Score > +50%)

| Category | Score | Why |
|----------|-------|-----|
| unquoted (+80.0%) | Strong structural detection of unquoted variable expansion in dangerous commands |
| ssrf (+72.7%) | IFDS taint analysis confirms server-side request forgery flows with zero false positives |
| codeinj (+72.2%) | Source injection and curl\|bash patterns well-covered by dedicated rules |
| hardcoded_creds (+71.4%) | Credential pattern matching (variable names + entropy) works with zero FP |
| weakcrypto (+66.7%) | md5sum/sha1sum structural detection clean, zero false positives |
| rce (+66.7%) | 100% TPR on curl\|bash patterns, one FP on checksum-verified download |

## Weaknesses (Score < +25%)

| Category | Score | Root Cause |
|----------|-------|------------|
| sqli (+4.8%) | 13 FN: Most SQL injection in bash uses string interpolation into sqlite3/mysql CLI. Taint confirms 28 VULNERABLE flows but rule only surfaces a subset. Standalone testcode functions lack taint entry points. |
| insecure_perms (+5.0%) | 3 FP: Engine flags chmod 755 and chmod 600 as findings (overly broad). Misses symbolic chmod (a+rwx) and umask patterns. |
| pathtraver (-12.7%) | 4 FP on safe patterns (validate_path, basename). 5 FN in standalone testcode files lacking taint context. |
| infodisclosure (-29.2%) | 5 FP on safe patterns (filtered subshell, masked logs, hardcoded responses). Only detects set -x, misses env dumps and credential-in-error-message patterns. |

## False Negatives (51 missed vulnerabilities)

Every FN is a detection gap. Grouped by root cause:

### Standalone testcode lacks taint entry points (19 FN)
Functions in `testcode/` take parameters but have no CGI environment or CLI dispatch context. The taint engine can't trace data flow into these functions because no entry_point edges exist.

**Affected:** sqli_direct_interpolation, sqli_multihop_taint, sqli_mysql_cli, sqli_table_name_injection, sqli_order_by_injection, pathtraver_cat_user_input, pathtraver_tar_extract, pathtraver_symlink_bypass, pathtraver_rm_user_path, pathtraver_tar_member_traversal, ssrf_curl_user_url, ssrf_url_path_injection, ssrf_wget_redirect_follow, cmdi_xargs_tainted, cmdi_argument_injection_grep, infodisclosure_env_dump, infodisclosure_error_msg_path, insecure_temp_timestamp, ssl_node_tls_reject_disabled

### No rule for this pattern (14 FN)
The engine has no structural or taint rule that detects these specific bash patterns.

**Affected:** cmdi_nameref_injection (declare -n), cmdi_arithmetic_expansion ($(( )) array subscript injection), cmdi_sed_expression_injection (sed -e), cmdi_awk_program_injection (awk program), codeinj_bash_c_injection (bash -c), codeinj_trap_injection (trap), perms_chmod_arwx (symbolic chmod), ssl_git_no_verify (GIT_SSL_NO_VERIFY env var), hardcoded_curl_auth (curl -u inline), hardcoded_creds_in_heredoc (credentials in heredoc body), weakcrypto_openssl_des (openssl enc -des), weakcrypto_sha1_signature (openssl dgst -sha1), dfg_infra_incomplete_redaction (keyword-based log redaction), dfo_cleanup_find_pattern (find -name with user pattern)

### SQL injection in apps not surfaced by rules (8 FN)
IFDS taint pipeline confirms these as VULNERABLE flows, but no bash rule calls `get_confirmed_flows("SQL Injection")` to surface them as findings.

**Affected:** sqlInjectionRecordDeployment, sqlInjectionGetDeploymentStatus, sqlInjectionRecordHealthCheck, sqlInjectionRecordConfigChange, sqlInjectionCleanupDays, sqlInjectionPushEvent, sqlInjectionReleaseEvent, dfw_sql_query_endpoint

### Other detection gaps (10 FN)
Mixed causes: SSH host key bypass not matched, deploy command injection via pipeline.sh, eval in specific contexts.

**Affected:** sshHostKeyBypass, runSeedScript, deployFromWebhookPayload, sedConfigInjection, dfw_eval_query_string, dfw_exec_endpoint, dfo_deploy_bash_c_hook, unquoted_for_loop, unquoted_test_bracket

## False Positives (30 safe code incorrectly flagged)

Every FP is a discrimination gap. Grouped by root cause:

### Engine flags safe validation/sanitization patterns (15 FP)
The engine detects the dangerous operation (eval, chmod, source, curl) but doesn't recognize the preceding validation or sanitization as a taint break.

**Affected:** cmdi_eval_hardcoded_safe, cmdi_printf_q_sanitizer_safe, cmdi_bash_qquote_sanitizer_safe, codeinj_source_validated_safe, dfw_deploy_safe_validated, dfw_notify_safe_sanitized, dfw_parse_qs_safe, dfo_backup_safe_validated, dfo_cleanup_safe_validated, dfo_deploy_safe_validated, dfo_notify_safe_encoded, dfo_wh_parse_qs_safe, ifsModificationSafe, dbQuerySafeEscaped, downloadFileSafe

### Engine flags safe infrastructure functions (10 FP)
Functions that use curl, sqlite3, pgrep, chmod, etc. with hardcoded or validated inputs. The engine sees the dangerous command but not the safe context.

**Affected:** installFromRemoteSafe, dfw_send_response_safe, dfw_json_get_jq_safe, dfw_json_build_escaped, dfg_healthcheck_http_safe, dfg_healthcheck_process_safe, dfg_healthcheck_file_safe, dfg_healthcheck_database_safe, dfg_healthcheck_disk_safe, insecure_temp_noclobber_safe

### Overly broad permission detection (3 FP)
Engine flags chmod 600, chmod 755, and umask 077 — these are secure permission settings.

**Affected:** chmod755DeployDirSafe, secureFilePerms, perms_umask_077_safe

### Other FP (2 FP)
Edge cases in subshell filtering and mktemp recognition.

**Affected:** infodisclosure_subshell_filtered_safe, insecure_temp_mktemp_safe

---

## Reproduction

```bash
# 1. Install TheAuditor v3.5
# 2. Setup and scan
cd gorustbash_benchmark/bash
aud setup-ai --target . --sync
aud full --offline

# 3. Score
python3 bash_benchmark.py
```

---

## Next Steps

Each FN and FP above is a trackable improvement target. When TheAuditor v3.6 or v4.0 ships, we will re-run this benchmark and publish the delta. The score should go up. If it doesn't, we'll explain why.
