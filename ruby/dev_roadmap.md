# Ruby SAST Benchmark Development Roadmap

## v0.2.0 (Released 2026-04-07)
- Authentication failure tests (CWE-287): JWT bypass, session fixation, Devise, Warden, OAuth
- Authorization failure tests (CWE-862): IDOR, Pundit, CanCanCan, tenant isolation, Action Policy
- All 27 categories expanded to 10+ vuln / 10+ safe minimum
- 573 total test cases (up from 318)

## v0.3.0 (Released 2026-04-07)
- All 27 categories expanded from 10/10 to 25/25 minimum (1,350 total tests)
- 777 new standalone test files added to testcode/
- Three-tier test design: direct, multi-hop (FN bait), discriminating near-miss (FP bait)
- Youden's J noise reduced 60%: each test case worth ±4% (down from ±10%)
- New patterns: Oj/Psych/YAML deserial variants, XSLT XXE, Ox tolerant, Regexp.union
  ReDoS, per-object Regexp.timeout (Ruby 3.2+), RE2 gem, hardcoded GCP/Twilio/PKCS12/
  PEM keys, Vault/SecretsManager/EJSON credential retrieval

## v0.3.1 (Released 2026-04-08)
- Anti-target leakage migration: all testcode files match OWASP Java/Go gold standard
- All 1,288 testcode files renamed to `benchmark_test_NNNNN.rb` (seeded shuffle)
- ALL comments stripped from test files (zero annotations, zero prose)
- 48 category-leaking function names replaced with domain-descriptive names
- Scoring switched from annotation-based to filename-based matching
- 62 apps/ entries excluded from testcode CSV (separate concern)
- 16 categories at 21-24 TP/TN pending backfill to 25/25

## v0.4.0 (Planned)
- Backfill 62 testcode replacements for apps/ entries (restore 25/25 all categories)
- ERB template extraction and analysis
- Haml/Slim template benchmark integration
- Liquid template injection patterns (CWE-1336)
- Ruby on Rails API-only application patterns
- Cross-file taint tracking test cases (multi-file snippets)

## Known Limitations
- dynmethod vs codeinj boundary: Some edge cases where metaprogramming overlaps with direct eval
- YAML.load behavior changed in Ruby 3.1/Psych 4 (safe by default). Benchmark marks YAML.load as TP conservatively
- Marshal.load has no safe form -- TN deserial cases use alternative serialization formats
- ERB templates (.erb files) not included in v0.1.0 -- Phase 2 work
