require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_securecookie_short_lived
def set_short_cookie(req)
  token = SecureRandom.hex(32)
  expires = (Time.now + 900).httpdate
  response = BenchmarkResponse.ok('session set')
  response.headers['Set-Cookie'] = "session=#{token}; Secure; HttpOnly; SameSite=Strict; Expires=#{expires}" # vuln-code-snippet safe-line ruby_securecookie_short_lived
  response
end
# vuln-code-snippet end ruby_securecookie_short_lived
