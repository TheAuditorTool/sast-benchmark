require_relative 'shared'

# vuln-code-snippet start ruby_csrf_head_method
def track_email_open(req)
  email_id = req.param('id').to_i
  # HEAD request (used for email tracking pixels) mutates state — no CSRF token required for HEAD
  db = get_db_connection
  db.execute("UPDATE emails SET opened_at = ? WHERE id = ?", [Time.now.to_s, email_id])  # vuln-code-snippet vuln-line ruby_csrf_head_method
  BenchmarkResponse.json({ result: '' })
end
# vuln-code-snippet end ruby_csrf_head_method
