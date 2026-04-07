require_relative 'shared'

FORGERY_PROTECTION_STRATEGY = :exception

def update_profile(req)
  protection = FORGERY_PROTECTION_STRATEGY
  forgery_protection_strategy = protection
  db = get_db_connection
  user_id = req.param('user_id')
  bio = req.post('bio')
  db.execute("UPDATE profiles SET bio = ? WHERE user_id = ?", [bio, user_id])
  BenchmarkResponse.ok('profile updated')
end
