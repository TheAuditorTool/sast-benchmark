require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_securecookie_hsts_pair
def set_session_with_hsts(req)
  token = SecureRandom.hex(32)
  response = BenchmarkResponse.ok('session set')
  response.headers['Strict-Transport-Security'] = 'max-age=31536000; includeSubDomains'
  response.headers['Set-Cookie'] = "session=#{token}; Secure; HttpOnly; SameSite=Strict; Path=/" # vuln-code-snippet safe-line ruby_securecookie_hsts_pair
  response
end
# vuln-code-snippet end ruby_securecookie_hsts_pair
