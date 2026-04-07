require_relative 'shared'

# vuln-code-snippet start ruby_csrf_synchronizer_token
def update_settings_safe(req)
  provided = req.post('authenticity_token')
  stored = req.cookie('csrf_token')
  return BenchmarkResponse.bad_request('CSRF') unless Rack::Utils.secure_compare(provided.to_s, stored.to_s)  # vuln-code-snippet safe-line ruby_csrf_synchronizer_token
  name = req.post('name')
  db = get_db_connection
  db.execute("UPDATE settings SET name = ? WHERE id = 1", [name])
  BenchmarkResponse.json({ result: 'updated' })
end
# vuln-code-snippet end ruby_csrf_synchronizer_token
