require_relative 'shared'

# vuln-code-snippet start ruby_csrf_skip_action
def delete_account(req)
  skip_before_action_csrf_verification = true  # vuln-code-snippet vuln-line ruby_csrf_skip_action
  db = get_db_connection
  user_id = req.param('user_id')
  db.execute("DELETE FROM users WHERE id = ?", [user_id])
  BenchmarkResponse.ok('account deleted')
end
# vuln-code-snippet end ruby_csrf_skip_action
