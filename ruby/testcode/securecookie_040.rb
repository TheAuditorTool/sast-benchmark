require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_securecookie_no_sensitive_data
def set_session(req)
  session_id = SecureRandom.uuid # vuln-code-snippet safe-line ruby_securecookie_no_sensitive_data
  response = BenchmarkResponse.ok('session set')
  response.headers['Set-Cookie'] = "session_id=#{session_id}; Secure; HttpOnly; SameSite=Strict; Path=/"
  response
end
# vuln-code-snippet end ruby_securecookie_no_sensitive_data
