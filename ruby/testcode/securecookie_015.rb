require_relative 'shared'

# vuln-code-snippet start ruby_securecookie_httponly_false
def set_cookie_no_httponly(req)
  token = req.param('token')
  response = BenchmarkResponse.ok('session set')
  response.headers['Set-Cookie'] = "session=#{token}; Secure; HttpOnly=false" # vuln-code-snippet vuln-line ruby_securecookie_httponly_false
  response
end
# vuln-code-snippet end ruby_securecookie_httponly_false
