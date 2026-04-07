require_relative 'shared'

def set_auth_cookie(req)
  token = req.param('token')
  cookie_opts = { value: token, secure: true, httponly: true, same_site: 'Strict', path: '/app' }
  header = Rack::Utils.add_cookie_to_header(nil, 'auth_token', cookie_opts)
  response = BenchmarkResponse.ok('auth cookie set')
  response.headers['Set-Cookie'] = header
  response
end
