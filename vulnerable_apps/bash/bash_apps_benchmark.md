# Bash Reference App Benchmarks

## Overview

5 intentionally vulnerable Bash applications with 191 ground truth entries across 13 CWE categories. Tests DevOps pipeline, webhook server, and infrastructure automation patterns where shell injection, SSRF, and insecure file operations are realistic attack surfaces.

## Apps

| App | Type | Files | Entries | Primary Patterns |
|-----|------|-------|---------|------------------|
| pipeline-manager | CI/CD pipeline orchestrator | 10 | 78 | SQL injection, command injection, SSRF, path traversal, insecure perms |
| securepipeline | Hardened CI/CD pipeline | 7 | 55 | TN-heavy: validated inputs, allowlists, quoted expansions |
| deepflow-webhook | HTTP webhook server | 8 | 28 | eval injection, SSRF, SQL injection, template injection |
| deepflow-ops | Operations suite | 5 | 20 | Command injection, SSRF, SQL injection |
| dataforge | Data pipeline | 4 | 10 | Info disclosure, command injection, SSRF |

## Ground Truth Format

`expectedresults-0.1.0.csv` uses the same format as the main benchmark CSVs:

```csv
# test name,category,real vulnerability,CWE
dfg_infra_incomplete_redaction,infodisclosure,true,532
dfg_healthcheck_http,infodisclosure,false,200
```

Test names map to inline `vuln-code-snippet` annotations in the source files:

```bash
# vuln-code-snippet start dfw_eval_query_string
    parse_query_string_eval "$QUERY_STRING" # vuln-code-snippet vuln-line dfw_eval_query_string
# vuln-code-snippet end dfw_eval_query_string
```

## Categories

| Category | CWE | Vuln | Safe | Total |
|----------|-----|------|------|-------|
| cmdi | 78 | 42 | 30 | 72 |
| sqli | 89 | 16 | 16 | 32 |
| codeinj | 94 | 11 | 4 | 15 |
| ssrf | 918 | 8 | 5 | 13 |
| unquoted | 78 | 7 | 4 | 11 |
| infodisclosure | 200 | 3 | 7 | 10 |
| pathtraver | 22 | 4 | 5 | 9 |
| insecure_perms | 732 | 2 | 5 | 7 |
| weakcrypto | 327 | 3 | 3 | 6 |
| ssl_bypass | 295 | 3 | 3 | 6 |
| rce | 94 | 2 | 3 | 5 |
| hardcoded_creds | 798 | 3 | 0 | 3 |
| insecure_temp | 377 | 1 | 1 | 2 |
| **Total** | | **105** | **86** | **191** |

TP/TN balance: 55/45. securepipeline is intentionally TN-heavy (demonstrates correct hardening patterns).

## Key Patterns

- **eval injection**: `eval "$user_input"`, `bash -c "$user_input"`, `source "$user_input"`
- **Unquoted expansion**: `rm $path` vs `rm "$path"` (word splitting / glob injection)
- **SQL via shell**: `sqlite3 "$DB" "SELECT * FROM t WHERE col = '$input'"` vs parameterized
- **SSRF via curl**: `curl "$user_url"` vs allowlisted URL check
- **Insecure permissions**: `chmod 666 "$file"` vs `chmod 600 "$file"`
- **Hardened counterparts**: securepipeline demonstrates every defense (allowlists, quoting, validation, integer checks)

## Annotation Key Prefixes

| Prefix | App |
|--------|-----|
| `dfg_` | dataforge |
| `dfo_` | deepflow-ops |
| `dfw_` | deepflow-webhook |
| `pm_` | pipeline-manager |
| `sp_` | securepipeline |
