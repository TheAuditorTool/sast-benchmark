require_relative 'shared'

def lookup_user_by_username(req)
  db = get_db_connection
  name = req.post('username')
  return BenchmarkResponse.bad_request('username required') if name.empty?

  rows = db.execute("SELECT id, username, email, created_at FROM users WHERE username = :name", name: name)
  user = rows.first
  return BenchmarkResponse.bad_request('user not found') unless user

  BenchmarkResponse.json({ id: user[0], username: user[1], email: user[2] })
end
