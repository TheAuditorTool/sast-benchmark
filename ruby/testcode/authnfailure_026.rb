require_relative 'shared'

# vuln-code-snippet start ruby_authn_session_no_regen
def login_no_session_regen(req, session)
  username = req.post('username')
  password = req.post('password')
  stored_password = 'correct-horse-battery-staple'
  return BenchmarkResponse.error('Unauthorized', 401) unless password == stored_password
  session[:user_id] = 42 # vuln-code-snippet vuln-line ruby_authn_session_no_regen
  session[:username] = username
  BenchmarkResponse.ok("Logged in as #{username}")
end
# vuln-code-snippet end ruby_authn_session_no_regen
