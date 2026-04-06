require_relative 'shared'

# vuln-code-snippet start ruby_securecookie_domain_wide
def set_cookie_domain_wide(req)
  token = req.param('token')
  response = BenchmarkResponse.ok('session set')
  response.headers['Set-Cookie'] = "session=#{token}; Domain=.example.com; Path=/" # vuln-code-snippet vuln-line ruby_securecookie_domain_wide
  response
end
# vuln-code-snippet end ruby_securecookie_domain_wide
