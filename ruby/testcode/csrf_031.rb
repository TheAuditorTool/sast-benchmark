require_relative 'shared'

# vuln-code-snippet start ruby_csrf_rack_middleware_bypass
def rack_sub_app_handler(req)
  action = req.post('action')
  user_id = req.param('user_id').to_i
  # This Rack endpoint is mounted at /api/legacy, below the Rails CSRF middleware
  # It processes state changes with no CSRF protection
  db = get_db_connection
  db.execute("UPDATE user_settings SET action_log = ? WHERE user_id = ?", [action, user_id])  # vuln-code-snippet vuln-line ruby_csrf_rack_middleware_bypass
  BenchmarkResponse.json({ result: 'processed' })
end
# vuln-code-snippet end ruby_csrf_rack_middleware_bypass
