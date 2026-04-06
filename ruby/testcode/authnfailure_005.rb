require_relative 'shared'

# vuln-code-snippet start ruby_authn_session_fixation
def login_no_reset(req)
  username = req.param('username')
  password = req.param('password')
  # authenticate user... session not reset
  BenchmarkResponse.ok("logged in: #{username}") # vuln-code-snippet vuln-line ruby_authn_session_fixation
end
# vuln-code-snippet end ruby_authn_session_fixation
