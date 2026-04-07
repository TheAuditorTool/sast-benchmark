require_relative 'shared'

def set_cookie_full_flags(req)
  token = req.param('token')
  response = BenchmarkResponse.ok('session set')
  response.headers['Set-Cookie'] = "session=#{token}; Secure; HttpOnly; SameSite=Strict"
  response
end
