# Rust Reference App Benchmarks

## Overview

8 intentionally vulnerable Rust applications with 119 ground truth entries across 14 CWE categories. Tests Rust-specific vulnerability patterns including unsafe memory operations, integer overflow, and framework-specific taint flows.

## Apps

| App | Framework | Entries | Primary Patterns |
|-----|-----------|---------|------------------|
| rocket_test | Rocket | 11 | SQLi, SSRF, XSS, path traversal, weak crypto |
| warp_test | Warp | 12 | SQLi, SSRF, XSS, command injection, weak crypto |
| rust_backend | actix-web | 17 | Path traversal, command injection, unsafe memory, regex DoS, deserialization |
| rust_taint_app | actix-web | 41 | SQLi, SSRF, command injection, cross-file taint flows |
| rust_calorie_app | actix-web | 12 | SQLi (format! vs parameterized), hardcoded credentials |
| rust_jobqueue | actix-web | 23 | SQLi, path traversal, command injection, weak random, JWT |
| anarchy_commerce | actix-web | 3 | SQLi, hardcoded credentials, input validation |
| deepflow-rust | actix-web | 0 | No annotations (complex taint flow app, not yet classified) |

## Ground Truth Format

`expectedresults-0.1.0.csv` uses the same format as the main benchmark CSVs:

```csv
# test name,category,real vulnerability,CWE
sqliRocketGetUser,sqli,true,89
sqliRocketGetUser2,sqli,false,89
```

Test names map to inline `vuln-code-snippet` annotations in the source files. Rust uses `target-line` instead of `vuln-line` for all annotations -- the CSV (not the annotation) determines vulnerability status.

## Categories

| Category | CWE | Vuln | Safe | Total |
|----------|-----|------|------|-------|
| sqli | 89 | 20 | 16 | 36 |
| cmdi | 78 | 11 | 5 | 16 |
| pathtraver | 22 | 11 | 1 | 12 |
| ssrf | 918 | 7 | 5 | 12 |
| memsafety | 119 | 7 | 4 | 11 |
| crypto | 327 | 5 | 4 | 9 |
| weakrand | 330 | 3 | 2 | 5 |
| xss | 79 | 2 | 2 | 4 |
| intoverflow | 190 | 2 | 1 | 3 |
| deserial | 502 | 2 | 1 | 3 |
| infodisclosure | 532 | 3 | 0 | 3 |
| hardcodedcreds | 798 | 2 | 0 | 2 |
| redos | 1333 | 1 | 1 | 2 |
| inputval | 20 | 1 | 0 | 1 |
| **Total** | | **77** | **42** | **119** |

TP/TN balance: 65/35. Skewed toward vulnerable because realistic apps tend to have more vulnerable than safe patterns.

## Key Patterns

- **Rust-specific CWEs**: `unsafe { *data.get_unchecked(offset) }` (memsafety), `a.wrapping_add(b)` (integer overflow), `serde_json::from_slice::<Value>` vs typed struct (deserialization)
- **SQL injection**: `format!("SELECT ... {}", user_input)` vs `conn.execute("SELECT ... ?1", [input])`
- **Multi-crate taint**: rust_jobqueue spans jobqueue-api, jobqueue-db, jobqueue-cli crates with cross-crate taint flows
- **Framework sources**: Rocket `Form<T>`, Warp `query::<T>()`, actix-web `web::Query<T>`

## Annotation Key Convention

Rust uses camelCase keys with the pattern `{category}{AppName}{Description}`:

```
sqliRocketGetUser     -> sqli category, rocket_test app
cmdiBackendShellExec  -> cmdi category, rust_backend app
memsafetyBackendBufferAlloc -> memsafety category, rust_backend app
```
