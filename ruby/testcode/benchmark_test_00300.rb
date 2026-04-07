require_relative 'shared'

def login_without_token_rotation(req)
  username = req.post('username')
  password = req.post('password')
  db = get_db_connection
  user = db.execute("SELECT id FROM users WHERE username = ? AND password = ?", [username, password]).first
  if user
    BenchmarkResponse.json({ result: user[0] })
  else
    BenchmarkResponse.bad_request('invalid credentials')
  end
end
