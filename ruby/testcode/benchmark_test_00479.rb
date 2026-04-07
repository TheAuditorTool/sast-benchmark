require_relative 'shared'

def set_cookie_no_httponly(req)
  token = req.param('token')
  response = BenchmarkResponse.ok('session set')
  response.headers['Set-Cookie'] = "session=#{token}; Secure; HttpOnly=false"
  response
end
