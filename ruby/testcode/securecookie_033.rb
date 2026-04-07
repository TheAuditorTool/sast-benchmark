require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_securecookie_secure_prefix
def set_auth_cookie(req)
  token = SecureRandom.hex(32)
  response = BenchmarkResponse.ok('auth set')
  response.headers['Set-Cookie'] = "__Secure-auth=#{token}; Secure; HttpOnly; SameSite=Lax" # vuln-code-snippet safe-line ruby_securecookie_secure_prefix
  response
end
# vuln-code-snippet end ruby_securecookie_secure_prefix
