require_relative 'shared'

def update_profile_ajax(req)
  user_id = req.param('user_id').to_i
  new_email = req.post('email')
  db = get_db_connection
  db.execute("UPDATE users SET email = ? WHERE id = ?", [new_email, user_id])
  BenchmarkResponse.json({ result: 'updated' })
end
