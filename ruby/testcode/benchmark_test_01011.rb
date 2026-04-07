require_relative 'shared'

def set_lax_session_cookie(req)
  user_id = req.post('user_id')
  cookie_val = "session=#{user_id}; SameSite=Lax; Secure; HttpOnly; Path=/"
  BenchmarkResponse.ok(cookie_val)
end
