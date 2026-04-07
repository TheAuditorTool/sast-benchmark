require_relative 'shared'

# vuln-code-snippet start ruby_csrf_put_no_token
def update_resource_put(req)
  resource_id = req.param('id').to_i
  data = req.post('data')
  # PUT verb skipped from CSRF protection — attacker can use CORS preflightless PUT
  db = get_db_connection
  db.execute("UPDATE resources SET data = ? WHERE id = ?", [data, resource_id])  # vuln-code-snippet vuln-line ruby_csrf_put_no_token
  BenchmarkResponse.json({ result: 'updated' })
end
# vuln-code-snippet end ruby_csrf_put_no_token
