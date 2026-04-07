require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_securecookie_clearance_safe
def set_remember_token(req)
  token = SecureRandom.hex(32)
  expires = (Time.now + 365 * 24 * 3600).httpdate
  response = BenchmarkResponse.ok('remember set')
  response.headers['Set-Cookie'] = "remember_token=#{token}; Secure; HttpOnly; SameSite=Lax; Expires=#{expires}" # vuln-code-snippet safe-line ruby_securecookie_clearance_safe
  response
end
# vuln-code-snippet end ruby_securecookie_clearance_safe
