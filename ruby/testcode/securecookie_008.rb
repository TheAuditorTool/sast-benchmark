require_relative 'shared'

# vuln-code-snippet start ruby_securecookie_rack_full
def set_cookie_full_flags(req)
  token = req.param('token')
  response = BenchmarkResponse.ok('session set')
  response.headers['Set-Cookie'] = "session=#{token}; Secure; HttpOnly; SameSite=Strict" # vuln-code-snippet safe-line ruby_securecookie_rack_full
  response
end
# vuln-code-snippet end ruby_securecookie_rack_full
