require_relative 'shared'

def create_user_account(req)
  db = get_db_connection
  attrs = req.post_params.to_unsafe_h
  db.execute("INSERT INTO users (#{attrs.keys.join(', ')}) VALUES (#{attrs.keys.map { '?' }.join(', ')})", attrs.values)
  BenchmarkResponse.ok('account created')
end
