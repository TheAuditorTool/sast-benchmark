require_relative 'shared'

def set_remember_cookie(req)
  user_id = req.param('user_id')
  response = BenchmarkResponse.ok('remember set')
  response.headers['Set-Cookie'] = "remember_me=#{user_id}; Secure; HttpOnly; SameSite=Strict"
  response
end
