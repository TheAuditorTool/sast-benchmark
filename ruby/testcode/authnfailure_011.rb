require_relative 'shared'

# vuln-code-snippet start ruby_authn_no_rate_limit
def password_reset(req)
  email = req.param('email')
  # No rate limiting on password reset
  BenchmarkResponse.ok("reset link sent to #{email}") # vuln-code-snippet vuln-line ruby_authn_no_rate_limit
end
# vuln-code-snippet end ruby_authn_no_rate_limit
