require_relative 'shared'

# vuln-code-snippet start ruby_csrf_no_token
def update_email(req)
  db = get_db_connection
  user_id = req.param('user_id')
  new_email = req.param('email')
  db.execute("UPDATE users SET email = ? WHERE id = ?", [new_email, user_id])  # vuln-code-snippet vuln-line ruby_csrf_no_token
  BenchmarkResponse.ok('email updated')
end
# vuln-code-snippet end ruby_csrf_no_token
