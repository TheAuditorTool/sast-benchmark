# PHP SAST Benchmark v0.3.2

## Purpose

The first public OWASP-style SAST benchmark for PHP. No equivalent exists from OWASP, NIST, or any other organization in the standard OWASP Benchmark format. This benchmark provides ground truth for measuring PHP static analysis tool accuracy.

## Test Case Inventory

25 CWE categories, 1,250 test cases (625 TP / 625 TN), 50/50 balance. All 25 categories meet the 25/25 TP/TN minimum floor.

### Tier 1: Core (High SAST Detectability)

| Category | CWE | Description | PHP-Specific |
|----------|-----|-------------|-------------|
| sqli | 89 | SQL Injection | PDO, mysqli, $wpdb, DB::raw, Doctrine |
| xss | 79 | Cross-Site Scripting | echo/print, Blade, Twig, WP escaping |
| cmdi | 78 | Command Injection | exec, system, passthru, shell_exec, proc_open |
| fileinclusion | 98 | File Inclusion (LFI/RFI) | include/require with tainted input |
| pathtraver | 22 | Path Traversal | file_get_contents, fopen, unlink |
| deserial | 502 | Insecure Deserialization | unserialize(), phar://, yaml_parse |
| ssrf | 918 | Server-Side Request Forgery | curl, file_get_contents($url), Guzzle |
| codeinj | 94 | Code Injection | eval(), assert(), preg_replace /e |

### Tier 2: PHP-Specific (Differentiators)

| Category | CWE | Description | PHP-Specific |
|----------|-----|-------------|-------------|
| typejuggling | 697 | Type Juggling | == vs === in auth, magic hashes |
| extract | 621 | Variable Extraction | extract($_POST), parse_str() |
| variablevars | 627 | Variable Variables | $$varname, ${$_GET['x']} |
| unsafereflect | 470 | Unsafe Reflection | new $className(), call_user_func |
| xxe | 611 | XML External Entity | simplexml, DOMDocument |
| fileupload | 434 | Unrestricted Upload | move_uploaded_file without validation |

### Tier 3: Standard Web

| Category | CWE | Description |
|----------|-----|-------------|
| redirect | 601 | Open Redirect |
| weakhash | 328 | Weak Hash (md5/sha1 for passwords) |
| weakrand | 330 | Weak Random (rand/mt_rand) |
| weakcipher | 327 | Weak Cipher (DES, RC4, ECB) |
| hardcodedcreds | 798 | Hardcoded Credentials |
| csrf | 352 | Cross-Site Request Forgery |
| headerinj | 113 | HTTP Header Injection |
| ldapi | 90 | LDAP Injection |
| securecookie | 614 | Insecure Cookie (missing Secure/HttpOnly) |
| massassign | 915 | Mass Assignment (Laravel $fillable) |
| ssti | 1336 | Server-Side Template Injection |

## Frameworks Covered

Testcode uses no framework (raw function signatures via `shared.php`). Framework-specific test cases (Laravel, Symfony, WordPress) live in `vulnerable_apps/php/` and are scored separately.

## Anti-Target Leakage Rules

Test files use generic `benchmark_test_NNNNN.php` naming with shuffled
numbering (seeded, reproducible). Files contain NO comments indicating
vulnerability category, CWE, or TP/TN status. This ensures SAST tools
must perform actual dataflow/AST analysis rather than text-matching
filenames or comments.

- 1 file = 1 test case
- Filename: `benchmark_test_NNNNN.php` (5-digit, seeded shuffle)
- Entry function: `benchmarkTestNNNNN(BenchmarkRequest $req): BenchmarkResponse`
- Ground truth lives in the CSV only -- never in source files
- No comments except where functionally required by the code

## Ground Truth

`expectedresults-0.3.2.csv` -- OWASP CSV format:

```csv
# test name,category,real vulnerability,CWE
BenchmarkTest00001,weakrand,false,330
BenchmarkTest00002,ssrf,true,918
```

## Scoring

See [SCORING.md](SCORING.md) for full methodology and tool-specific instructions.

## Validation

```bash
python scripts/validate_php.py
```

Runs L1-L5 fidelity checks: structural integrity, naming convention,
schema validation, anti-target-leakage, and scoring pipeline readiness.

## PHP-Unique CWEs

These 6 categories have no equivalent in Go, Rust, or Bash benchmarks:

1. **CWE-98 (File Inclusion)**: `include($_GET['page'])` -- PHP's include/require can execute arbitrary files, including remote URLs. No other benchmarked language has this vulnerability class.

2. **CWE-697 (Type Juggling)**: `if ($hash == "0e12345")` -- PHP's loose comparison operator `==` treats strings starting with "0e" as scientific notation equal to zero. Critical in authentication bypasses.

3. **CWE-621 (Variable Extraction)**: `extract($_POST)` -- Overwrites local variables from user input. Unique to PHP's `extract()` and legacy `register_globals`.

4. **CWE-627 (Variable Variables)**: `$$varname = $value` -- PHP allows variable names to be dynamically constructed. Can overwrite security-critical variables.

5. **CWE-470 (Unsafe Reflection)**: `new $className()` -- PHP allows class instantiation from strings. Combined with autoloading, enables arbitrary class instantiation.

6. **CWE-1336 (SSTI)**: `$twig->createTemplate($input)->render()` -- Template engines (Twig, Blade, Smarty) can execute arbitrary code when templates are constructed from user input.
