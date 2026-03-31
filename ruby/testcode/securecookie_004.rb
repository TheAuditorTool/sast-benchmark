require_relative 'shared'

# vuln-code-snippet start ruby_securecookie_rack_secure
def set_session(req)
  session_id = req.param('session_id')
  cookie_opts = { value: session_id, secure: true, httponly: true, same_site: 'Strict', path: '/' }
  header = Rack::Utils.add_cookie_to_header(nil, 'session', cookie_opts)  # vuln-code-snippet safe-line ruby_securecookie_rack_secure
  response = BenchmarkResponse.ok('session started')
  response.headers['Set-Cookie'] = header
  response
end
# vuln-code-snippet end ruby_securecookie_rack_secure
