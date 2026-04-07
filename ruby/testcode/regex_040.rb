require_relative 'shared'

# vuln-code-snippet start ruby_regex_fixed_pattern_only
def handle_fixed_pattern_only(req)
  slug = req.param('slug')
  result = /\A[a-z0-9_]{1,50}\z/.match(slug) # vuln-code-snippet safe-line ruby_regex_fixed_pattern_only
  BenchmarkResponse.json({ valid: !result.nil? })
end
# vuln-code-snippet end ruby_regex_fixed_pattern_only
