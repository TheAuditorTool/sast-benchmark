# PHP SAST Benchmark Changelog

## v0.3.0 (2026-04-07)

Expansion release: all 25 categories reach 25 TP + 25 TN minimum.

- 694 new standalone test cases (346 TP / 348 TN) across 24 categories
- Total: 1,256 test cases (628 TP / 628 TN), 25 CWE categories, 4 frameworks
- All categories now have minimum 25 TP + 25 TN (50 total per category)
- sqli unchanged at 28/28 (already above floor)
- Youden precision improved: one misclassification now equals ±4% score swing (was ±10%)
- Legacy PHP patterns annotated with version comments (PHP 5.x/7.x)
- New patterns cover: mail() 5th-arg injection, PHAR chain variants, magic hash bypasses,
  JWT/CSRF combinations, PHAR stream alternatives, PHP 8 type juggling vectors,
  cloud metadata SSRF, gopher:// protocol SSRF, Twig createTemplate SSTI,
  DOM text node XSS defense, GD re-encode upload defense, and many more
- 1,138 standalone testcode files (up from 444)
- CSV version bump: expectedresults-0.2.0.csv → expectedresults-0.3.0.csv

## v0.2.0 (2026-04-07)

Expansion release: all 25 categories reach 10 TP + 10 TN minimum.

- 193 new standalone test cases (96 TP / 97 TN) across 21 categories
- Total: 562 test cases (282 TP / 280 TN), 25 CWE categories, 4 frameworks
- All categories now have minimum 10 TP + 10 TN (20 total per category)
- New patterns include: igbinary deserialization, sodium AEAD ciphers, Latte/Mustache SSTI, fsockopen/SoapClient SSRF, class_alias reflection, CHIPS cookie partitioning, PBKDF2/Argon2 key derivation, double-submit CSRF, GD image reprocessing, JSON Schema mass assignment defense, and more
- 444 standalone testcode files (up from 251)
- CSV version bump: expectedresults-0.1.0.csv -> expectedresults-0.2.0.csv

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
