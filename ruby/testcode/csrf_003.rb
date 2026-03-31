require_relative 'shared'

FORGERY_PROTECTION_STRATEGY = :null_session

# vuln-code-snippet start ruby_csrf_null_session
def update_profile(req)
  protection = FORGERY_PROTECTION_STRATEGY
  forgery_protection_strategy = protection  # vuln-code-snippet vuln-line ruby_csrf_null_session
  db = get_db_connection
  user_id = req.param('user_id')
  bio = req.post('bio')
  db.execute("UPDATE profiles SET bio = ? WHERE user_id = ?", [bio, user_id])
  BenchmarkResponse.ok('profile updated')
end
# vuln-code-snippet end ruby_csrf_null_session
