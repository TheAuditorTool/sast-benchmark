require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_securecookie_long_lived_no_rotation
def set_persistent_auth(req)
  token = SecureRandom.hex(32)
  expires = (Time.now + 365 * 24 * 3600).httpdate
  response = BenchmarkResponse.ok('auth set')
  response.headers['Set-Cookie'] = "auth=#{token}; Secure; HttpOnly; SameSite=Strict; Expires=#{expires}" # vuln-code-snippet vuln-line ruby_securecookie_long_lived_no_rotation
  response
end
# vuln-code-snippet end ruby_securecookie_long_lived_no_rotation
