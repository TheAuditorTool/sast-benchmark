require_relative 'shared'

def set_cookie_samesite_none(req)
  token = req.param('token')
  response = BenchmarkResponse.ok('session set')
  response.headers['Set-Cookie'] = "session=#{token}; SameSite=None"
  response
end
