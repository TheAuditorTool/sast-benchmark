# Ruby SAST Benchmark Changelog

## v0.3.2 (2026-04-08)

### Standardization Release

File naming convention standardized, comments stripped, CWE ordering shuffled to eliminate positional bias. No test case changes -- ground truth identical to v0.3.1.

- CSV version bump to 0.3.2
- Removed all legacy CSV versions (0.1.0, 0.2.0, 0.3.0, 0.3.1)
- Updated all documentation references to 0.3.2
- validate_ruby.py updated to point to 0.3.2

### Final State
- 1,288 test cases (unchanged)
- 27 CWE categories (unchanged)
- 644/644 TP/TN balance (unchanged)

---

## v0.3.1 (2026-04-08)

Anti-target leakage migration: testcode files match OWASP Java/Go gold standard.

- All 1,288 testcode files renamed to `benchmark_test_NNNNN.rb` (seeded shuffle)
- ALL comments stripped: annotations, prose, trailing — zero `#` in any test file
- 48 category-leaking function names replaced with domain-descriptive names
- Scoring switched from annotation-based to filename-based matching (same as Go/Java)
- CSV keys now `BenchmarkTestNNNNN` format matching filenames
- 62 apps/ entries excluded from testcode CSV (apps/ untouched, separate concern)
- 1,288 test cases (644 TP / 644 TN), 27 CWE categories
- rename_map.json preserved for traceability

## v0.3.0 (2026-04-07)

Major statistical hardening: all 27 categories expanded from 10/10 to 25/25 minimum.

- 1,350 test cases across 27 CWE categories (675 TP / 675 TN, exact 50/50 balance)
- Every category now has exactly 25 vulnerable + 25 safe test cases (up from 10/10)
- 777 new standalone test files added to testcode/
- Three-tier test design: Tier 1 (direct), Tier 2 (multi-hop/indirect, FN bait),
  Tier 3 (discriminating near-miss, FP bait)
- Youden's J noise reduced 60%: each test worth ±4% (down from ±10% at 10/10)
- New TP patterns: Oj compat/rails/wab deserial, YAML.load_file traversal+deserial,
  cookie/Redis Marshal.load, Psych.load_stream, JSON.load object injection; hardcoded
  GCP keys, Twilio tokens, PKCS12 passwords, PEM private key heredocs, Azure/MongoDB
  credentials; Nokogiri dtdload/recover, REXML full document, XSLT from user, Ox
  tolerant; ReDoS via user-controlled patterns, alternation loops, lookahead loops,
  backreference catastrophic backtracking, Regexp.union with user patterns
- New TN patterns: Oj strict/hash mode, Psych.safe_load empty permitted_classes,
  schema-bound Protobuf/Avro/Thrift; Vault/AWS Secrets Manager/GCP Secret Manager/
  Azure KeyVault/Infisical/EJSON credential retrieval; Nokogiri nonet+nodtdload,
  REXML entity_expansion_limit=0, Ox strict/hash mode, HTML5 parser (DTD-immune);
  Regexp.timeout= (Ruby 3.2+), per-object timeout, RE2 gem, Regexp.escape,
  possessive/atomic quantifiers
- All APIs verified against MITRE CWE 4.19.1 and Ruby stdlib/gem documentation
- YAML.load TPs annotated: "TP: vulnerable in Ruby < 3.1 / Psych < 4; marked conservatively"
- Regexp.timeout TN patterns annotated: "# Ruby 3.2+"

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
