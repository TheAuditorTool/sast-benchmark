require_relative 'shared'

def set_session(req)
  session_id = req.param('session_id')
  cookie_opts = { value: session_id, secure: true, httponly: true, same_site: 'Strict', path: '/' }
  header = Rack::Utils.add_cookie_to_header(nil, 'session', cookie_opts)
  response = BenchmarkResponse.ok('session started')
  response.headers['Set-Cookie'] = header
  response
end
