require_relative 'shared'

# vuln-code-snippet start ruby_securecookie_plain_session_id
def set_session(req)
  user_id = req.param('user_id')
  response = BenchmarkResponse.ok('session set')
  response.headers['Set-Cookie'] = "session_id=user_#{user_id}_#{Time.now.to_i}; Secure; HttpOnly; SameSite=Strict" # vuln-code-snippet vuln-line ruby_securecookie_plain_session_id
  response
end
# vuln-code-snippet end ruby_securecookie_plain_session_id
