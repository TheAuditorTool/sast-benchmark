require_relative 'shared'

# vuln-code-snippet start ruby_sqli_string_interp
def get_user_profile(req)
  db = get_db_connection
  id = req.param('id')
  rows = db.execute("SELECT * FROM users WHERE id = #{id}")  # vuln-code-snippet vuln-line ruby_sqli_string_interp
  user = rows.first
  return BenchmarkResponse.bad_request('user not found') unless user
  BenchmarkResponse.json({ id: user[0], name: user[1], email: user[2] })
end
# vuln-code-snippet end ruby_sqli_string_interp
