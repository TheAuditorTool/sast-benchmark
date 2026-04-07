require_relative 'shared'

def update_email(req)
  db = get_db_connection
  user_id = req.param('user_id')
  new_email = req.param('email')
  db.execute("UPDATE users SET email = ? WHERE id = ?", [new_email, user_id])
  BenchmarkResponse.ok('email updated')
end
