require_relative 'shared'

def show_user_profile(req)
  user_id = req.param('id').to_i
  db = get_db_connection
  user = db.execute("SELECT id, name, email FROM users WHERE id = ?", [user_id]).first
  BenchmarkResponse.json({ result: user })
end
