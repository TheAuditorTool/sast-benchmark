require_relative 'shared'

# vuln-code-snippet start ruby_securecookie_remember_unencrypted
def set_remember_cookie(req)
  user_id = req.param('user_id')
  response = BenchmarkResponse.ok('remember set')
  response.headers['Set-Cookie'] = "remember_me=#{user_id}; Secure; HttpOnly; SameSite=Strict" # vuln-code-snippet vuln-line ruby_securecookie_remember_unencrypted
  response
end
# vuln-code-snippet end ruby_securecookie_remember_unencrypted
