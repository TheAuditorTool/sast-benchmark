# PHP Reference App Benchmarks

## Overview

4 intentionally vulnerable PHP applications with 118 ground truth entries. Covers vanilla PHP, Laravel, Symfony, and WordPress plugin patterns.

## Apps

| App | Framework | Files | Entries | Primary Patterns |
|-----|-----------|-------|---------|------------------|
| vuln_blog | Vanilla PHP | 7 | 40 | Raw SQL, XSS, CSRF, type juggling, deserialization |
| laravel_api | Laravel | 5 | 30 | Eloquent raw queries, mass assignment, SSRF, SSTI |
| symfony_app | Symfony | 4 | 24 | DQL/QueryBuilder injection, XXE, LDAP, unsafe reflection |
| wp_plugin | WordPress | 4 | 24 | wpdb queries, nonce checks, extract(), file inclusion |

## Ground Truth Format

`expectedresults-0.1.0.csv` uses the same format as the main benchmark CSVs:

```csv
# test name,category,real vulnerability,CWE
vb_sqli_login,sqli,true,89
vb_sqli_login_safe,sqli,false,89
```

Test names map to inline `vuln-code-snippet` annotations in the source files.

## Categories

| Category | CWE | Vuln | Safe | Total |
|----------|-----|------|------|-------|
| sqli | 89 | 13 | 13 | 26 |
| xss | 79 | 7 | 7 | 14 |
| pathtraver | 22 | 4 | 4 | 8 |
| hardcodedcreds | 798 | 3 | 3 | 6 |
| csrf | 352 | 3 | 3 | 6 |
| xxe | 611 | 2 | 2 | 4 |
| weakrand | 330 | 2 | 2 | 4 |
| typejuggling | 697 | 2 | 2 | 4 |
| ssti | 1336 | 2 | 2 | 4 |
| ssrf | 918 | 2 | 2 | 4 |
| redirect | 601 | 2 | 2 | 4 |
| massassign | 915 | 2 | 2 | 4 |
| fileupload | 434 | 2 | 2 | 4 |
| fileinclusion | 98 | 2 | 2 | 4 |
| cmdi | 78 | 2 | 2 | 4 |
| weakhash | 328 | 1 | 1 | 2 |
| variablevars | 627 | 1 | 1 | 2 |
| unsafereflect | 470 | 1 | 1 | 2 |
| securecookie | 614 | 1 | 1 | 2 |
| ldapi | 90 | 1 | 1 | 2 |
| headerinj | 113 | 1 | 1 | 2 |
| extract | 621 | 1 | 1 | 2 |
| deserial | 502 | 1 | 1 | 2 |
| codeinj | 94 | 1 | 1 | 2 |
| **Total** | | **59** | **59** | **118** |

## Key Patterns

- **PHP-unique CWEs**: type juggling (`==` vs `===`), `extract()` variable injection, `$$variable` variables, file inclusion via `include($user_input)`
- **Framework ORMs**: Laravel `DB::raw()` vs `DB::select("... ?", [val])`, Symfony DQL concat vs parameters
- **WordPress APIs**: `$wpdb->query("...$var")` vs `$wpdb->prepare("...%s", $var)`, `wp_verify_nonce()`
- **Template injection**: `eval('$output = "' . $rendered . '"')` vs `Blade::render($template, $data)`

## Annotation Key Prefixes

| Prefix | App |
|--------|-----|
| `vb_` | vuln_blog |
| `la_` | laravel_api |
| `sy_` | symfony_app |
| `wp_` | wp_plugin |
