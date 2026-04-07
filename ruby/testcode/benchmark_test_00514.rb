require_relative 'shared'

def unsubscribe_user(req)
  user_id = req.param('user_id').to_i
  db = get_db_connection
  db.execute("DELETE FROM subscriptions WHERE user_id = ?", [user_id])
  BenchmarkResponse.json({ result: 'unsubscribed' })
end
