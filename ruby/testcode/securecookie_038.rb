require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_securecookie_rotation_on_auth
def authenticate(req)
  token = SecureRandom.hex(32) # vuln-code-snippet safe-line ruby_securecookie_rotation_on_auth
  response = BenchmarkResponse.ok('authenticated')
  response.headers['Set-Cookie'] = "session=#{token}; Secure; HttpOnly; SameSite=Strict; Path=/"
  response
end
# vuln-code-snippet end ruby_securecookie_rotation_on_auth
