require_relative 'shared'

# vuln-code-snippet start ruby_csrf_no_cookie_state
def stateless_api_action(req)
  api_key = req.header('X-API-Key')
  return BenchmarkResponse.bad_request('missing API key') unless api_key
  db = get_db_connection
  key_record = db.execute("SELECT user_id FROM api_keys WHERE key_hash = ?", [Digest::SHA256.hexdigest(api_key)]).first
  return BenchmarkResponse.bad_request('invalid key') unless key_record
  # No session cookie — CSRF cannot target stateless API key auth
  BenchmarkResponse.json({ result: key_record[0] })  # vuln-code-snippet safe-line ruby_csrf_no_cookie_state
end
# vuln-code-snippet end ruby_csrf_no_cookie_state
