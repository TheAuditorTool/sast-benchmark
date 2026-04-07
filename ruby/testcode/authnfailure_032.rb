require_relative 'shared'

# vuln-code-snippet start ruby_authn_concurrent_session
def login_no_session_invalidation(req, session, db)
  username = req.post('username')
  password = req.post('password')
  user = db.execute('SELECT id FROM users WHERE username = ? AND password_digest = ?', [username, password]).first
  return BenchmarkResponse.error('Unauthorized', 401) unless user
  session[:user_id] = user[0] # vuln-code-snippet vuln-line ruby_authn_concurrent_session
  BenchmarkResponse.ok('Login successful')
end
# vuln-code-snippet end ruby_authn_concurrent_session
