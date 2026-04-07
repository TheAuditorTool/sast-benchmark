require_relative 'shared'

def track_email_open(req)
  email_id = req.param('id').to_i
  db = get_db_connection
  db.execute("UPDATE emails SET opened_at = ? WHERE id = ?", [Time.now.to_s, email_id])
  BenchmarkResponse.json({ result: '' })
end
