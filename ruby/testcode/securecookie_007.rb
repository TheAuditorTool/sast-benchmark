require_relative 'shared'

# vuln-code-snippet start ruby_securecookie_samesite_none
def set_cookie_samesite_none(req)
  token = req.param('token')
  response = BenchmarkResponse.ok('session set')
  response.headers['Set-Cookie'] = "session=#{token}; SameSite=None" # vuln-code-snippet vuln-line ruby_securecookie_samesite_none
  response
end
# vuln-code-snippet end ruby_securecookie_samesite_none
