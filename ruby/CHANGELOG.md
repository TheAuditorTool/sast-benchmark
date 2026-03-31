# Ruby SAST Benchmark Changelog

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
