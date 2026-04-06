# Ruby SAST Benchmark Development Roadmap

## v0.2.0 (Released 2026-04-07)
- Authentication failure tests (CWE-287): JWT bypass, session fixation, Devise, Warden, OAuth
- Authorization failure tests (CWE-862): IDOR, Pundit, CanCanCan, tenant isolation, Action Policy
- All 27 categories expanded to 10+ vuln / 10+ safe minimum
- 573 total test cases (up from 318)

## v0.3.0 (Planned)
- ERB template extraction and analysis
- Haml/Slim template benchmark integration
- Liquid template injection patterns (CWE-1336)
- Ruby on Rails API-only application patterns

## Known Limitations
- dynmethod vs codeinj boundary: Some edge cases where metaprogramming overlaps with direct eval
- YAML.load behavior changed in Ruby 3.1/Psych 4 (safe by default). Benchmark marks YAML.load as TP conservatively
- Marshal.load has no safe form -- TN deserial cases use alternative serialization formats
- ERB templates (.erb files) not included in v0.1.0 -- Phase 2 work
