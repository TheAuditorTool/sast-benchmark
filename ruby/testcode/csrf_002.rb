require_relative 'shared'

require 'rack/utils'

# vuln-code-snippet start ruby_csrf_verified_token
def update_email_verified(req)
  db = get_db_connection
  session_token = req.cookie('csrf_token')
  request_token = req.post('csrf_token')
  return BenchmarkResponse.error('CSRF check failed', 403) unless Rack::Utils.secure_compare(session_token, request_token)  # vuln-code-snippet safe-line ruby_csrf_verified_token
  new_email = req.post('email')
  user_id = req.post('user_id')
  db.execute("UPDATE users SET email = ? WHERE id = ?", [new_email, user_id])
  BenchmarkResponse.ok('email updated')
end
# vuln-code-snippet end ruby_csrf_verified_token
