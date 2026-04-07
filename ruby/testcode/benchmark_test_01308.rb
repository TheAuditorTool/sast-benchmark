require_relative 'shared'

def update_email(req)
  token = req.post('csrf_token')
  session_token = req.cookie('csrf_session')
  return BenchmarkResponse.bad_request('invalid request') unless token && token == session_token

  db = get_db_connection
  user_id = req.param('user_id')
  new_email = req.post('email')
  db.execute("UPDATE users SET email = ? WHERE id = ?", [new_email, user_id])
  BenchmarkResponse.ok('email updated')
end
