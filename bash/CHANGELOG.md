# Bash SAST Benchmark — Changelog

## v0.4.0 (2026-04-07)

526 test cases across 20 categories, 5 applications, 56 shell scripts. **CWE expansion + 10/10 floor release.**

### 4 New CWE Categories (MITRE pre-flight verified)
- **loginjection (CWE-117)**: 10 TP + 10 TN. User input in log files without control char neutralization. echo/printf/logger/tee/heredoc. Safe: printf %q, tr -d, jq, allowlist, base64.
- **privilege_escalation (CWE-250)**: 10 TP + 10 TN. Execution with unnecessary privileges. sudo/su/docker --privileged/nsenter/pkexec/setcap with user-controlled args. Safe: hardcoded commands, capability drops, rootless containers.
- **dos (CWE-770)**: 10 TP + 10 TN. Allocation of resources without limits. Unbounded loops, fork bombs, tar bombs, unlimited xargs. Safe: ulimit, timeout, cgroup limits, bounded retries.
- **cleartext_tx (CWE-319)**: 10 TP + 10 TN. Cleartext transmission. http://, FTP, telnet, nc, mysql/redis without TLS. Safe: https://, SFTP, SSH, ncat --ssl, --ssl-mode=REQUIRED.

**CWE Pre-Flight**: CWE-269 (Privilege Escalation) rejected — Class-level, mapping DISCOURAGED. Replaced with CWE-250 (Base-level, Allowed). CWE-400 (DoS) rejected — Class-level, mapping DISCOURAGED. Replaced with CWE-770 (Base-level, Allowed). Sources: cwe.mitre.org definitions 250, 269, 400, 770.

### All 16 Existing Categories Expanded to 10/10 Floor
- insecure_temp: 4/4 -> 10/10 (+6V/+6S)
- auth_bypass: 4/4 -> 10/10 (+6V/+6S)
- rce: 5/5 -> 10/10 (+5V/+5S)
- weakrand: 5/5 -> 10/10 (+5V/+5S)
- race_condition: 5/5 -> 10/10 (+5V/+5S)
- insecure_perms: 5/7 -> 10/10 (+5V/+3S)
- weakcrypto: 6/6 -> 10/10 (+4V/+4S)
- ssl_bypass: 6/7 -> 10/10 (+4V/+3S)
- hardcoded_creds: 7/7 -> 10/10 (+3V/+3S)
- infodisclosure: 6/9 -> 10/10 (+4V/+1S)
- pathtraver: 9/9 -> 10/10 (+1V/+1S)

### Metrics
- 526 test cases (263 TP / 263 TN) — **exact 50.0% / 50.0% balance**
- 20 CWE categories (was 16)
- All 20 categories at minimum 10V/10S
- 170 new test cases added (88 TP + 82 TN)

### Metadata Updates
- `expectedresults-0.4.0.csv` — new ground truth (526 entries)
- `bash_benchmark.py` — GROUND_TRUTH_PATH, RULE_MAP, SINK_MAP updated for 4 new categories
- `scripts/validate_bash.py` — CSV reference updated, CWEs 250/319/770 added to VALID_CWES
- `scripts/convert_theauditor.py` — VULN_TYPE_TO_CWE entries for 4 new categories

---

## v0.3.1 (2026-03-22)

356 test cases across 16 categories, 5 applications, 52 shell scripts. **OWASP rebalancing release.**

### SARIF Scoring Pipeline
- `scripts/convert_theauditor.py` now supports all 3 languages (Go, Bash, Rust) via combined RULE_MAP
- For Bash/Rust: converter scans vuln-code-snippet annotations to resolve (file, line) findings to test case keys
- `scripts/score_sarif.py` now supports key-based matching via `properties.testCaseKey` (Bash/Rust) in addition to filename matching (Go)
- Bash benchmark now has two scoring paths: direct DB (`bash_benchmark.py`) and SARIF pipeline (`convert_theauditor.py` -> `score_sarif.py`)

### Ground Truth Migration (YAML -> CSV)
- Migrated from `bash_ground_truth.yml` to `expectedresults-0.3.1.csv` (OWASP standard format)
- CSV format matches Go/Java/Python benchmarks: `test name,category,real vulnerability,CWE`
- Deleted `bash_ground_truth.yml` — single source of truth is now the CSV
- Rewrote `bash_benchmark.py` parser and `scripts/validate_bash.py` for CSV input
- All L1-L5 fidelity checks pass on CSV

### OWASP Foundation Feedback Response

The OWASP Foundation reviewed v0.3 and identified the 68/32 TP/TN split as the "biggest methodological weakness" — a tool that blindly flags everything scores 68% TPR for free. This release addresses that feedback directly.

### New Application
- **apps/securepipeline/** — CI/CD pipeline (7 files, 55 TN-only cases). A secure version of pipeline-manager demonstrating proper input validation, parameterized queries, quoting, allowlisting, and safe API patterns.

### TP/TN Rebalancing
- Added 91 new safe (TN) test cases across securepipeline app and existing testcode/ files
- TP/TN split improved from 68/32 (v0.3) to **49/51** (v0.3.1)
- 13 of 16 categories now at exact 50/50 TP/TN balance
- 3 categories slightly TN-heavy (acceptable): insecure_perms (42/58), ssl_bypass (46/54), infodisclosure (40/60)
- Safe pattern taxonomy expanded: dead variable, input-only-echoed, regex-validated discrete args, getopts structured parsing, mapfile array population, single-quoted SSH/docker commands, select menus, declare -F dispatch, jq JSON escaping, process-substitution-for-reading, and more

### Safe Pattern Categories Added (inspired by Go benchmark's paired-twin approach)
- **Dead variable**: user input read but constant used at sink (Go's #1 safe pattern)
- **Hardcoded command with user as discrete arg**: ping/head/grep with validated user argument
- **Structured parsing**: getopts, mapfile -t, IFS read -r
- **Container isolation**: single-quoted SSH/docker exec commands
- **Dynamic dispatch via declare -F**: verify function existence before calling
- **Declarative parsing**: grep/cut metadata extraction instead of source
- **Docker secrets**: /run/secrets/ mount instead of hardcoded credentials
- **jq JSON escaping**: safe JSON construction for webhook payloads

### Quality Assurance
- Every new TN classification verified as genuinely safe (not toy examples)
- Zero duplicate keys (agent-verified against 356-entry YAML)
- Annotation count matches YAML count exactly (356/356)
- Per-category balance verified programmatically

### New CWE Categories (CWE Top 25 gap closure)
- **weakrand (CWE-330)**: 5 TP + 5 TN. $RANDOM is 15-bit LCG — session tokens, OTPs, API keys. Safe: /dev/urandom, openssl rand, mktemp, python secrets.
- **race_condition (CWE-362)**: 5 TP + 5 TN. TOCTOU on lock files, PID files, stat-then-source, mkdir-chmod gap. Safe: flock, noclobber, mkdir-as-lock, install -m.
- **auth_bypass (CWE-287/306)**: 4 TP + 4 TN. SKIP_AUTH env bypass, empty credential comparison, missing webhook signature, timing-vulnerable compare. Safe: HMAC verification gate, empty-check rejection, mandatory auth, constant-time compare.

### Source Code Hint Purge (CONTRIBUTING.md compliance)
- Stripped ~278 inline hint comments from 38 files (28 apps + 10 testcode)
- Removed: `# VULNERABLE`, `# SAFE:`, `# TAINT FLOW:`, `# Taint:`, `# DANGEROUS`, `# CRITICAL`, `# BAD:`, `# Safe:`, `# Vulnerable:`, `# COMMAND INJECTION`, `# CONTAINS INTENTIONAL VULNERABILITIES`, section markers, header blocks
- Per CONTRIBUTING.md line 15: "No vulnerability hints. The CSV is the only ground truth."
- Technical explanations preserved (only classification prefix stripped)
- All vuln-code-snippet annotations (start/end/vuln-line/safe-line) preserved
- Verified: 0 hint matches remaining, L1-L5 fidelity PASS, 356/356 annotations intact

### Key & Function Name Normalization (OWASP compliance)
- Renamed 184 annotation keys: 78 camelCase→snake_case + 132 classification suffix stripped
- Stripped `_safe`, `_unsafe`, `_insecure`, `_vulnerable` suffixes from all keys and function names
- Renamed 83 function definitions to remove classification hints (e.g. `authenticate_safe()` → `authenticate()`, `db_query_unsafe()` → `db_query_interpolated()`)
- 4 collision pairs resolved with technique-descriptive names: `_eval`/`_declare`, `_interpolated`/`_escaped`, `_dynamic`/`_literal`
- Cross-file call sites updated (parse_query_string_eval in deepflow-webhook)
- Zero camelCase keys remaining, zero `_safe`/`_unsafe` function names remaining
- L1-L5 fidelity PASS, 356/356, 0 duplicate keys

### Known Limitations (remaining from v0.3)
- Baseline scorecard established (see `baseline_theauditor_tool_score.md`)
- All new CWE category TP cases are tagged [EXPECTED_FN] — no engine rules exist yet for these patterns
- IFDS taint analysis still only consumed for Command Injection in the reference tool
- Self-authored benchmark (bias risk documented)
- Scoring script still hardcoded to specific tool's database schema (SARIF interface planned)

---

## v0.3 (2026-03-19)

237 test cases across 13 categories, 4 applications, 42 shell scripts, 7,716 lines.

### Applications
- **apps/pipeline-manager/** — DevOps CI/CD Pipeline Manager (10 scripts). SSH/SCP deployment, Docker, Kubernetes, SQLite tracking, webhook handlers, backup/restore, database migrations.
- **apps/deepflow-webhook/** — HTTP webhook server (8 files). socat/netcat listener, GitHub/GitLab/Slack webhook handling, deploy/notify/exec/query endpoints.
- **apps/deepflow-ops/** — Operations suite with SAFE_MODE toggle (7 files). Backup, cleanup, deploy, notify, webhook handler. Each script has safe + vulnerable code paths.
- **apps/dataforge/** — Data pipeline scripts (4 files). Backup, deploy, healthcheck, Terraform infrastructure setup.
- **testcode/** — 13 purpose-built CWE test files with TP/TN pairs.

### Test Case Summary
- 161 vulnerable (TP), 76 safe (TN) — 68/32 split
- 13 CWE categories: cmdi (74), codeinj (24), sqli (27), pathtraver (16), infodisclosure (14), ssrf (13), unquoted (13), hardcoded_creds (11), ssl_bypass (11), insecure_perms (9), weakcrypto (9), insecure_temp (8), rce (8)

### Development Phases
- **Phase 0**: Foundation — copied pipeline manager, wrote 158 initial test cases
- **Phase 1**: Fix known errors — verified arithmetic expansion IS vulnerable (agent was wrong), moved heredoc annotation
- **Phase 2**: Tier 1 coverage — added 14 test cases (find -exec, eval+$(curl), source <(), argument injection, multi-step eval chain, TOCTOU, credentials in heredoc)
- **Phase 3**: Tier 2 coverage — added 7 test cases (${var@Q} sanitizer, NODE_TLS env, tar member traversal, ORDER BY injection)
- **Phase 4**: Documentation polish — updated all stats, detection coverage matrix
- **Phase 6A**: Added deepflow-webhook (28 test cases). JSON field eval, email header injection, double eval templates, SSRF via git clone.
- **Phase 6B**: Added deepflow-ops (20 test cases). SAFE_MODE dual-path design, env var as command, git branch injection, stdin-fed eval/SQL, cross-service taint.
- **Phase 6C**: Added dataforge (10 test cases). Incomplete keyword redaction (CWE-532), JSON injection, 5 safe healthcheck functions. 2 automated classification errors corrected via manual verification.
- **Phase 6D**: Final documentation polish.

### Quality Assurance
- Every test case classification verified against actual bash semantics (no assumptions — read the code, verify the behavior)
- 2 agent classification errors caught and corrected (healthcheck.sh check_process and check_database are SAFE)
- 1 arithmetic expansion classification challenged by automated analysis and upheld after manual verification (bash arithmetic DOES execute commands via array subscript injection)
- Automated validation: `scripts/validate_bash.py` — 237/237 ground truth + annotation match, zero errors

### Known Limitations
- TP/TN split 68/32 (OWASP Java is 52/48)
- IFDS taint analysis only consumed for Command Injection in the reference tool — 91 confirmed data flows for SQL Injection, SSRF, Path Traversal, Information Disclosure available but not yet surfaced as rule findings
- Self-authored benchmark (bias risk documented)
