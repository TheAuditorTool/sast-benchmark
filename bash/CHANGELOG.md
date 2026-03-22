# Bash SAST Benchmark — Changelog

## v0.3.1 (2026-03-22)

356 test cases across 16 categories, 5 applications, 52 shell scripts. **OWASP rebalancing release.**

### Ground Truth Migration (YAML -> CSV)
- Migrated from `bash_ground_truth.yml` to `expectedresults-0.3.1.csv` (OWASP standard format)
- CSV format matches Go/Java/Python benchmarks: `test name,category,real vulnerability,CWE`
- Deleted `bash_ground_truth.yml` — single source of truth is now the CSV
- Rewrote `bash_benchmark.py` parser and `scripts/validate_bash.py` for CSV input
- All L1-L5 fidelity checks pass on CSV

### OWASP Foundation Feedback Response

The OWASP Foundation reviewed v0.3 and identified the 68/32 TP/TN split as the "biggest methodological weakness" — a tool that blindly flags everything scores 68% TPR for free. This release addresses that feedback directly.

### New Application
- **apps/securepipeline/** — Hardened CI/CD pipeline (7 files, 55 TN-only safe cases). A deliberately secure version of pipeline-manager demonstrating proper input validation, parameterized queries, quoting, allowlisting, and safe API patterns. Every function includes inline comments explaining WHY it is safe.

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

### Known Limitations (remaining from v0.3)
- No baseline scorecard yet (awaiting first SAST tool run)
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
- No baseline scorecard yet (awaiting first SAST tool run)
- IFDS taint analysis only consumed for Command Injection in the reference tool — 91 confirmed data flows for SQL Injection, SSRF, Path Traversal, Information Disclosure available but not yet surfaced as rule findings
- Self-authored benchmark (bias risk documented)
