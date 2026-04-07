require_relative 'shared'

def update_profile(req)
  db = get_db_connection
  user_id = req.param('user_id')
  attrs = req.post_params.to_unsafe_h
  columns = attrs.keys.map { |k| "#{k} = ?" }.join(', ')
  db.execute("UPDATE users SET #{columns} WHERE id = ?", attrs.values + [user_id])
  BenchmarkResponse.ok('profile updated')
end
