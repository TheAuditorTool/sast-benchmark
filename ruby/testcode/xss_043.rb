require_relative 'shared'

# vuln-code-snippet start ruby_xss_csp_json_only
def get_user_data_api(req)
  user_id = req.param('id').to_i
  db = get_db_connection
  row = db.execute("SELECT id, name FROM users WHERE id = ?", [user_id]).first
  # JSON response with explicit content-type — no HTML rendering, XSS not applicable
  BenchmarkResponse.json({ id: row&.dig(0), name: row&.dig(1) })  # vuln-code-snippet safe-line ruby_xss_csp_json_only
end
# vuln-code-snippet end ruby_xss_csp_json_only
