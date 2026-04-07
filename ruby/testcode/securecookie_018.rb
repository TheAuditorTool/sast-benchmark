require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_securecookie_lax_sensitive
def set_payment_session(req)
  token = SecureRandom.hex(32)
  response = BenchmarkResponse.ok('payment session set')
  response.headers['Set-Cookie'] = "payment_session=#{token}; Secure; HttpOnly; SameSite=Lax" # vuln-code-snippet vuln-line ruby_securecookie_lax_sensitive
  response
end
# vuln-code-snippet end ruby_securecookie_lax_sensitive
