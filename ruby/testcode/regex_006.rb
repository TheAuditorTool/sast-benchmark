require_relative 'shared'

SLUG_PATTERN = /\A[a-z0-9\-]+\z/

# vuln-code-snippet start ruby_regex_length_limit
def validate_slug(req)
  input = req.param('slug')
  truncated = input[0, 1000]  # vuln-code-snippet safe-line ruby_regex_length_limit
  matched = SLUG_PATTERN.match(truncated)
  BenchmarkResponse.ok(matched ? 'valid slug' : 'invalid slug')
end
# vuln-code-snippet end ruby_regex_length_limit
