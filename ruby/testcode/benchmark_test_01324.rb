require_relative 'shared'

PROFILE_FIELDS = %w[display_name bio location website].freeze

def update_profile(req)
  db = get_db_connection
  user_id = req.param('user_id')
  permitted = PROFILE_FIELDS.each_with_object({}) { |f, h| h[f] = req.post(f) if req.post(f) }
  columns = permitted.keys.map { |k| "#{k} = ?" }.join(', ')
  db.execute("UPDATE users SET #{columns} WHERE id = ?", permitted.values + [user_id])
  BenchmarkResponse.ok('profile updated')
end
