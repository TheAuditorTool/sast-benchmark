require_relative 'shared'

def delete_account(req)
  skip_before_action_csrf_verification = true
  db = get_db_connection
  user_id = req.param('user_id')
  db.execute("DELETE FROM users WHERE id = ?", [user_id])
  BenchmarkResponse.ok('account deleted')
end
