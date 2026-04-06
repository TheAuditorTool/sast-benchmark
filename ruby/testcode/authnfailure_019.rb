require_relative 'shared'

# vuln-code-snippet start ruby_authn_oauth_no_state
def oauth_callback(req)
  code = req.param('code')
  # No state parameter validation
  BenchmarkResponse.ok("oauth code: #{code}") # vuln-code-snippet vuln-line ruby_authn_oauth_no_state
end
# vuln-code-snippet end ruby_authn_oauth_no_state
