require_relative 'shared'

# vuln-code-snippet start ruby_csrf_get_state_change
def unsubscribe_user(req)
  user_id = req.param('user_id').to_i
  # GET request deletes subscription — no CSRF protection on GET, but state changes
  db = get_db_connection
  db.execute("DELETE FROM subscriptions WHERE user_id = ?", [user_id])  # vuln-code-snippet vuln-line ruby_csrf_get_state_change
  BenchmarkResponse.json({ result: 'unsubscribed' })
end
# vuln-code-snippet end ruby_csrf_get_state_change
