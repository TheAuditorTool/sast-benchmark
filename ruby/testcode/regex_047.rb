require_relative 'shared'

# vuln-code-snippet start ruby_regex_no_user_pattern
def handle_no_user_pattern(req)
  slug = req.param('slug')
  valid = /\A[a-z0-9\-]+\z/.match?(slug) # vuln-code-snippet safe-line ruby_regex_no_user_pattern
  BenchmarkResponse.json({ valid: valid })
end
# vuln-code-snippet end ruby_regex_no_user_pattern
