require_relative 'shared'

# vuln-code-snippet start ruby_securecookie_permanent
def set_remember_me(req)
  user_id = req.param('user_id')
  expires = (Time.now + 60 * 60 * 24 * 365).httpdate
  response = BenchmarkResponse.ok('remember me set')
  response.headers['Set-Cookie'] = "remember_token=#{user_id}; Expires=#{expires}"  # vuln-code-snippet vuln-line ruby_securecookie_permanent
  response
end
# vuln-code-snippet end ruby_securecookie_permanent
