require_relative 'shared'

def login_no_session_invalidation(req, session, db)
  username = req.post('username')
  password = req.post('password')
  user = db.execute('SELECT id FROM users WHERE username = ? AND password_digest = ?', [username, password]).first
  return BenchmarkResponse.error('Unauthorized', 401) unless user
  session[:user_id] = user[0]
  BenchmarkResponse.ok('Login successful')
end
