require_relative 'shared'

# vuln-code-snippet start ruby_csrf_ajax_no_header
def update_profile_ajax(req)
  user_id = req.param('user_id').to_i
  new_email = req.post('email')
  # No X-CSRF-Token header validation for AJAX endpoint
  db = get_db_connection
  db.execute("UPDATE users SET email = ? WHERE id = ?", [new_email, user_id])  # vuln-code-snippet vuln-line ruby_csrf_ajax_no_header
  BenchmarkResponse.json({ result: 'updated' })
end
# vuln-code-snippet end ruby_csrf_ajax_no_header
