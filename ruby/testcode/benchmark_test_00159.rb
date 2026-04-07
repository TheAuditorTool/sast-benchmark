require_relative 'shared'

def lookup_user_by_email(req)
  email = req.post('email')
  return BenchmarkResponse.bad_request('missing email') if email.empty?
  db = get_db_connection
  rows = db.execute("SELECT id, name, role FROM users WHERE email = ?", [email])
  return BenchmarkResponse.bad_request('no match') if rows.empty?
  user = rows.first
  BenchmarkResponse.json({ id: user[0], name: user[1], role: user[2] })
end
