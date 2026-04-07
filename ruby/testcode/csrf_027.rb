require_relative 'shared'

# vuln-code-snippet start ruby_csrf_token_no_rotate
def login_without_token_rotation(req)
  username = req.post('username')
  password = req.post('password')
  db = get_db_connection
  user = db.execute("SELECT id FROM users WHERE username = ? AND password = ?", [username, password]).first
  if user
    # Session established but CSRF token NOT rotated — old token still valid
    # vuln-code-snippet vuln-line ruby_csrf_token_no_rotate
    BenchmarkResponse.json({ result: user[0] })
  else
    BenchmarkResponse.bad_request('invalid credentials')
  end
end
# vuln-code-snippet end ruby_csrf_token_no_rotate
