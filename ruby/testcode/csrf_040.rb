require_relative 'shared'

# vuln-code-snippet start ruby_csrf_post_only_safe
def show_user_profile(req)
  user_id = req.param('id').to_i
  db = get_db_connection
  # GET endpoint — read-only, no state change, CSRF not applicable
  user = db.execute("SELECT id, name, email FROM users WHERE id = ?", [user_id]).first  # vuln-code-snippet safe-line ruby_csrf_post_only_safe
  BenchmarkResponse.json({ result: user })
end
# vuln-code-snippet end ruby_csrf_post_only_safe
