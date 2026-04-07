require_relative 'shared'

# vuln-code-snippet start ruby_csrf_rotate_on_login
def login_with_rotation(req)
  username = req.post('username')
  password = req.post('password')
  db = get_db_connection
  user = db.execute("SELECT id FROM users WHERE username = ? AND password = ?", [username, password]).first
  if user
    new_session_id = SecureRandom.hex(32)
    new_csrf_token = SecureRandom.hex(32)
    BenchmarkResponse.json({ result: new_csrf_token })  # vuln-code-snippet safe-line ruby_csrf_rotate_on_login
  else
    BenchmarkResponse.bad_request('invalid credentials')
  end
end
# vuln-code-snippet end ruby_csrf_rotate_on_login
