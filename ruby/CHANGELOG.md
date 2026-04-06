# Ruby SAST Benchmark Changelog

## v0.2.0 (2026-04-07)

Major expansion: all categories to 10/10 minimum, two new CWE categories.

- 573 test cases across 27 CWE categories (285 TP / 288 TN, ~50/50 balance)
- New CWE categories: authentication failure (CWE-287), authorization failure (CWE-862)
- All 27 categories now have minimum 10 vulnerable + 10 safe test cases
- 255 new test files added to testcode/
- Ruby-specific auth patterns: JWT verification bypass, session fixation, Devise
  confirmable/lockable, Rack::Attack rate limiting, Pundit/CanCanCan authorization,
  IDOR patterns, tenant isolation
- Expanded coverage: weak ciphers (Blowfish, 3DES, RC2, static keys), ReDoS
  (possessive quantifiers, atomic groups, Regexp.timeout), log injection (Syslog
  format strings, structured logging), LDAP injection (bind_as, modify DN),
  dynamic method dispatch (method_missing, Struct.new), header injection
  (Content-Disposition, Cache-Control, Link header), unsafe reflection
  (constantize, method chains, namespace checks), and more
- CWE numbers verified against MITRE CWE v4.19.1
- validate_ruby.py updated for new categories and CWEs

## v0.1.0 (2026-03-31)

Initial release.

- 318 test cases across 25 CWE categories (159 TP / 159 TN, 50/50 balance)
- 3 framework targets: Raw Ruby/Rack, Rails 7, Sinatra 3
- Ruby-specific CWE categories: mass assignment (CWE-915), unsafe reflection
  (CWE-470), SSTI via ERB (CWE-1336), dynamic method dispatch (CWE-94),
  file inclusion via load/require (CWE-98), ReDoS (CWE-1333)
- 3 annotated applications: rack_app (24 cases), rails_api (26 cases),
  sinatra_app (12 cases)
- 256 standalone testcode files across 25 categories
- SARIF 2.1.0 scoring via annotation-based matching
- L1-L6 fidelity validation via validate_ruby.py
- TheAuditor converter support via convert_theauditor.py
