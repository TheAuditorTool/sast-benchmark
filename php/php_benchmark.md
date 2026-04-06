# PHP SAST Benchmark v0.2.0

## Purpose

The first public OWASP-style SAST benchmark for PHP. No equivalent exists from OWASP, NIST, or any other organization in the standard OWASP Benchmark format. This benchmark provides ground truth for measuring PHP static analysis tool accuracy.

## Test Case Inventory

25 CWE categories, 562 test cases (282 TP / 280 TN), 50/50 balance.

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

| Framework | Version Target | Usage |
|-----------|---------------|-------|
| Raw PHP/PDO | PHP 5.x - 8.x | testcode + vuln_blog app |
| Laravel | 8.x - 11.x patterns | laravel_api app |
| WordPress | 5.x - 6.x patterns | wp_plugin app |
| Symfony | 5.x - 7.x patterns | symfony_app app |

## Applications

| App | Framework | Test Cases | Description |
|-----|-----------|------------|-------------|
| vuln_blog | Raw PHP/PDO | 40 | Blog with auth, posts, comments, uploads, search |
| laravel_api | Laravel-style | 30 | REST API with Eloquent ORM, Blade templates |
| wp_plugin | WordPress | 24 | Plugin with $wpdb, nonces, shortcodes, AJAX |
| symfony_app | Symfony-style | 24 | Forms with Doctrine, Twig, LDAP auth |

## Annotation Format

Test cases are marked with `vuln-code-snippet` comments (same format as Rust and Bash benchmarks):

```php
// vuln-code-snippet start php_sqli_pdo_concat
function getUser(PDO $pdo, BenchmarkRequest $req): BenchmarkResponse {
    $id = $req->param('id');
    $query = "SELECT * FROM users WHERE id = " . $id; // vuln-code-snippet vuln-line php_sqli_pdo_concat
    $result = $pdo->query($query);
    return BenchmarkResponse::ok(json_encode($result->fetchAll()));
}
// vuln-code-snippet end php_sqli_pdo_concat
```

- `vuln-code-snippet start KEY` -- Opens a test case
- `vuln-code-snippet end KEY` -- Closes a test case
- `vuln-code-snippet vuln-line KEY` -- Marks the vulnerable line (TP test cases)
- `vuln-code-snippet safe-line KEY` -- Marks the safe line (TN test cases)

## Ground Truth

`expectedresults-0.2.0.csv` -- OWASP CSV format:

```csv
# test name,category,real vulnerability,CWE
php_sqli_pdo_concat,sqli,true,89
php_sqli_pdo_prepare,sqli,false,89
```

## Scoring

See [SCORING.md](SCORING.md) for full methodology and tool-specific instructions.

## Validation

```bash
python scripts/validate_php.py
```

Runs L1-L5 fidelity checks matching the Bash/Rust validation standard.

## PHP-Unique CWEs

These 6 categories have no equivalent in Go, Rust, or Bash benchmarks:

1. **CWE-98 (File Inclusion)**: `include($_GET['page'])` -- PHP's include/require can execute arbitrary files, including remote URLs. No other benchmarked language has this vulnerability class.

2. **CWE-697 (Type Juggling)**: `if ($hash == "0e12345")` -- PHP's loose comparison operator `==` treats strings starting with "0e" as scientific notation equal to zero. Critical in authentication bypasses.

3. **CWE-621 (Variable Extraction)**: `extract($_POST)` -- Overwrites local variables from user input. Unique to PHP's `extract()` and legacy `register_globals`.

4. **CWE-627 (Variable Variables)**: `$$varname = $value` -- PHP allows variable names to be dynamically constructed. Can overwrite security-critical variables.

5. **CWE-470 (Unsafe Reflection)**: `new $className()` -- PHP allows class instantiation from strings. Combined with autoloading, enables arbitrary class instantiation.

6. **CWE-1336 (SSTI)**: `$twig->createTemplate($input)->render()` -- Template engines (Twig, Blade, Smarty) can execute arbitrary code when templates are constructed from user input.
