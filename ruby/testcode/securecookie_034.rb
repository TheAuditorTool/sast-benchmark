require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_securecookie_strict_samesite
def set_session(req)
  token = SecureRandom.hex(32)
  response = BenchmarkResponse.ok('session set')
  response.headers['Set-Cookie'] = "session=#{token}; Secure; HttpOnly; SameSite=Strict; Path=/api" # vuln-code-snippet safe-line ruby_securecookie_strict_samesite
  response
end
# vuln-code-snippet end ruby_securecookie_strict_samesite
