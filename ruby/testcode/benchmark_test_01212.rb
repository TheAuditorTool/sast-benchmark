require_relative 'shared'

def set_remember_me(req)
  user_id = req.param('user_id')
  expires = (Time.now + 60 * 60 * 24 * 365).httpdate
  response = BenchmarkResponse.ok('remember me set')
  response.headers['Set-Cookie'] = "remember_token=#{user_id}; Expires=#{expires}"
  response
end
