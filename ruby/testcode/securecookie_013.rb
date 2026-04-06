require_relative 'shared'

# vuln-code-snippet start ruby_securecookie_no_expire
def set_permanent_cookie(req)
  token = req.param('token')
  response = BenchmarkResponse.ok('session set')
  expires = (Time.now + 365 * 24 * 3600).httpdate
  response.headers['Set-Cookie'] = "session=#{token}; Expires=#{expires}" # vuln-code-snippet vuln-line ruby_securecookie_no_expire
  response
end
# vuln-code-snippet end ruby_securecookie_no_expire
