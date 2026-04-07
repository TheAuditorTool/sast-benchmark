require_relative 'shared'

MAX_FAILURES = 5

# vuln-code-snippet start ruby_authn_lockout_devise
def login_with_lockout(req, db)
  username = req.post('username')
  password = req.post('password')
  row = db.execute('SELECT id, password_digest, failed_attempts FROM users WHERE username = ?', [username]).first
  return BenchmarkResponse.error('Unauthorized', 401) unless row
  user_id, stored_hash, failures = row
  return BenchmarkResponse.error('Account locked', 423) if failures >= MAX_FAILURES # vuln-code-snippet safe-line ruby_authn_lockout_devise
  unless password == stored_hash
    db.execute('UPDATE users SET failed_attempts = failed_attempts + 1 WHERE id = ?', [user_id])
    return BenchmarkResponse.error('Unauthorized', 401)
  end
  db.execute('UPDATE users SET failed_attempts = 0 WHERE id = ?', [user_id])
  BenchmarkResponse.ok("Welcome #{username}")
end
# vuln-code-snippet end ruby_authn_lockout_devise
