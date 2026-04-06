require_relative 'shared'

# vuln-code-snippet start ruby_authn_reset_session
def login_with_reset(req)
  username = req.param('username')
  # authenticate user...
  # reset_session -- prevents session fixation
  session_id = SecureRandom.hex(32) # vuln-code-snippet safe-line ruby_authn_reset_session
  BenchmarkResponse.ok("logged in: #{username}")
end
# vuln-code-snippet end ruby_authn_reset_session
