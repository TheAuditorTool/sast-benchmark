require_relative 'shared'

def get_user_profile(req)
  id = req.param('id')
  return BenchmarkResponse.bad_request('missing id') if id.empty?
  conn = get_db_connection
  result = conn.exec_params("SELECT id, username, email, created_at FROM users WHERE id = $1", [id])
  return BenchmarkResponse.bad_request('user not found') if result.ntuples == 0
  row = result.first
  BenchmarkResponse.json({ id: row['id'], username: row['username'], email: row['email'], created_at: row['created_at'] })
end
