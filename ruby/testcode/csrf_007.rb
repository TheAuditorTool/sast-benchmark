require_relative 'shared'

# vuln-code-snippet start ruby_csrf_no_check
def change_password(req)
  db = get_db_connection
  user_id = req.post('user_id')
  new_pass = req.post('password')
  db.execute("UPDATE users SET password = ? WHERE id = ?", [new_pass, user_id])  # vuln-code-snippet vuln-line ruby_csrf_no_check
  BenchmarkResponse.ok('password changed')
end
# vuln-code-snippet end ruby_csrf_no_check
