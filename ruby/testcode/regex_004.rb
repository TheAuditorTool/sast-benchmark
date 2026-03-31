require_relative 'shared'

EMAIL_PATTERN = /\A[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}\z/

# vuln-code-snippet start ruby_regex_flat_pattern
def validate_email(req)
  email = req.param('email')
  matched = EMAIL_PATTERN.match(email)  # vuln-code-snippet safe-line ruby_regex_flat_pattern
  BenchmarkResponse.ok(matched ? 'valid' : 'invalid')
end
# vuln-code-snippet end ruby_regex_flat_pattern
