# Ruby SAST Benchmark Development Roadmap

## v0.2.0 (Planned)
- Devise authentication pattern tests (CWE-287, CWE-306)
- Pundit authorization bypass patterns (CWE-862)
- ActiveStorage file handling patterns
- rack-attack bypass patterns
- Additional Rails 7+ Turbo/Hotwire patterns

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
