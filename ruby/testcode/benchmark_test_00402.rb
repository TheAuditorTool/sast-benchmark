require_relative 'shared'

def change_password(req)
  db = get_db_connection
  user_id = req.post('user_id')
  new_pass = req.post('password')
  db.execute("UPDATE users SET password = ? WHERE id = ?", [new_pass, user_id])
  BenchmarkResponse.ok('password changed')
end
