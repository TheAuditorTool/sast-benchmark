require_relative 'shared'

def set_permanent_cookie(req)
  token = req.param('token')
  response = BenchmarkResponse.ok('session set')
  expires = (Time.now + 365 * 24 * 3600).httpdate
  response.headers['Set-Cookie'] = "session=#{token}; Expires=#{expires}"
  response
end
