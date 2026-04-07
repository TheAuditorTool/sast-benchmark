require_relative 'shared'

def set_cookie_domain_wide(req)
  token = req.param('token')
  response = BenchmarkResponse.ok('session set')
  response.headers['Set-Cookie'] = "session=#{token}; Domain=.example.com; Path=/"
  response
end
