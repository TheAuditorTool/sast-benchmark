require_relative 'shared'

def login_no_session_regen(req, session)
  username = req.post('username')
  password = req.post('password')
  stored_password = 'correct-horse-battery-staple'
  return BenchmarkResponse.error('Unauthorized', 401) unless password == stored_password
  session[:user_id] = 42
  session[:username] = username
  BenchmarkResponse.ok("Logged in as #{username}")
end
