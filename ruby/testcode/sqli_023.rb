require_relative 'shared'

# vuln-code-snippet start ruby_sqli_named_bind
def lookup_user_by_username(req)
  db = get_db_connection
  name = req.post('username')
  return BenchmarkResponse.bad_request('username required') if name.empty?

  rows = db.execute("SELECT id, username, email, created_at FROM users WHERE username = :name", name: name)  # vuln-code-snippet safe-line ruby_sqli_named_bind
  user = rows.first
  return BenchmarkResponse.bad_request('user not found') unless user

  BenchmarkResponse.json({ id: user[0], username: user[1], email: user[2] })
end
# vuln-code-snippet end ruby_sqli_named_bind
