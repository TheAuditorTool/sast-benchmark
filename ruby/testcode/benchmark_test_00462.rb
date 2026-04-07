require_relative 'shared'

def get_user_data_api(req)
  user_id = req.param('id').to_i
  db = get_db_connection
  row = db.execute("SELECT id, name FROM users WHERE id = ?", [user_id]).first
  BenchmarkResponse.json({ id: row&.dig(0), name: row&.dig(1) })
end
