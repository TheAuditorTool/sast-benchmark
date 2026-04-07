require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_securecookie_session_only
def set_session(req)
  token = SecureRandom.hex(32)
  response = BenchmarkResponse.ok('session set')
  response.headers['Set-Cookie'] = "session=#{token}; Secure; HttpOnly; SameSite=Strict" # vuln-code-snippet safe-line ruby_securecookie_session_only
  response
end
# vuln-code-snippet end ruby_securecookie_session_only
