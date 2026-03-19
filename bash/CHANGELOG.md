# Bash SAST Benchmark — Changelog

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
