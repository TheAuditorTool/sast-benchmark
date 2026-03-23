# PHP SAST Benchmark Changelog

## v0.1.0 (2026-03-23)

Initial release.

- 369 test cases across 25 CWE categories (186 TP / 183 TN, 50/50 balance)
- 4 framework targets: Raw PHP/PDO, Laravel, WordPress, Symfony
- Both modern (PHP 8.x) and legacy (PHP 5.x/7.x) patterns
- 6 PHP-unique CWE categories: file inclusion (CWE-98), type juggling (CWE-697), variable extraction (CWE-621), variable variables (CWE-627), unsafe reflection (CWE-470), SSTI (CWE-1336)
- 4 annotated applications: vuln_blog (40 cases), laravel_api (30 cases), wp_plugin (24 cases), symfony_app (24 cases)
- 251 standalone testcode files across 25 categories
- SARIF 2.1.0 scoring via annotation-based matching
- L1-L5 fidelity validation via validate_php.py
- TheAuditor converter support via convert_theauditor.py
