require_relative 'shared'

def set_session_cookie(req)
  session_id = req.param('session_id')
  response = BenchmarkResponse.ok('session set')
  response.headers['Set-Cookie'] = Rack::Utils.add_cookie_to_header(
    nil, 'session', { value: session_id, same_site: 'Strict', secure: true, http_only: true }
  )
  response
end
