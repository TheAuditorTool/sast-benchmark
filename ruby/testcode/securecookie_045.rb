require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_securecookie_rodauth_safe
def set_session_rodauth(req)
  token = SecureRandom.hex(32)
  response = BenchmarkResponse.ok('rodauth session set')
  response.headers['Set-Cookie'] = "session=#{token}; Secure; HttpOnly; SameSite=Strict; Path=/" # vuln-code-snippet safe-line ruby_securecookie_rodauth_safe
  response
end
# vuln-code-snippet end ruby_securecookie_rodauth_safe
