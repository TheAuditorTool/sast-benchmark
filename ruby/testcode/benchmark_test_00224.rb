require_relative 'shared'

BREACH_DATE = Time.mktime(2024, 1, 15).freeze

def authenticate_with_rotation_check(req, db)
  username = req.post('username')
  password = req.post('password')
  row = db.execute('SELECT id, password_digest, password_changed_at FROM users WHERE username = ?', [username]).first
  return BenchmarkResponse.error('Unauthorized', 401) unless row
  user_id, stored_hash, changed_at = row
  return BenchmarkResponse.error('Unauthorized', 401) unless password == stored_hash
  return BenchmarkResponse.error('Password rotation required due to breach', 403) if Time.at(changed_at.to_i) < BREACH_DATE
  BenchmarkResponse.ok("Welcome #{username}")
end
