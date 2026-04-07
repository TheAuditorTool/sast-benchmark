require_relative 'shared'

def rack_sub_app_handler(req)
  action = req.post('action')
  user_id = req.param('user_id').to_i
  db = get_db_connection
  db.execute("UPDATE user_settings SET action_log = ? WHERE user_id = ?", [action, user_id])
  BenchmarkResponse.json({ result: 'processed' })
end
