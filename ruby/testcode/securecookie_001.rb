require_relative 'shared'

# vuln-code-snippet start ruby_securecookie_no_flags
def set_session(req)
  session_id = req.param('session_id')
  response = BenchmarkResponse.ok('session started')
  response.headers['Set-Cookie'] = "session=#{session_id}"  # vuln-code-snippet vuln-line ruby_securecookie_no_flags
  response
end
# vuln-code-snippet end ruby_securecookie_no_flags
