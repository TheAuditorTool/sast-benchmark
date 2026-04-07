require_relative 'shared'

# vuln-code-snippet start ruby_authn_session_regen_login
def login_with_session_regen(req, session)
  username = req.post('username')
  password = req.post('password')
  stored_password = 'correct-horse-battery-staple'
  return BenchmarkResponse.error('Unauthorized', 401) unless password == stored_password
  session.clear # vuln-code-snippet safe-line ruby_authn_session_regen_login
  session[:user_id] = 42
  session[:username] = username
  BenchmarkResponse.ok("Logged in as #{username}")
end
# vuln-code-snippet end ruby_authn_session_regen_login
