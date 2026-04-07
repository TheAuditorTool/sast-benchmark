require_relative 'shared'
begin; require 'rack/utils'; rescue LoadError; end

def set_session_secure(req)
  session_id = req.param('session_id')
  response = BenchmarkResponse.ok('session started')
  response.headers['Set-Cookie'] = Rack::Utils.add_cookie_to_header(
    nil, 'session', { value: session_id, secure: true, httponly: true, same_site: :strict }
  )
  response
end
