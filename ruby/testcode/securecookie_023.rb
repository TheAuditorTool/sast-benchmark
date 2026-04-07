require_relative 'shared'
require 'securerandom'

PRODUCTION = false

# vuln-code-snippet start ruby_securecookie_http_allowed
def set_auth_cookie(req)
  token = SecureRandom.hex(32)
  secure_flag = PRODUCTION ? 'Secure; ' : ''
  response = BenchmarkResponse.ok('auth set')
  response.headers['Set-Cookie'] = "auth=#{token}; #{secure_flag}HttpOnly; SameSite=Strict" # vuln-code-snippet vuln-line ruby_securecookie_http_allowed
  response
end
# vuln-code-snippet end ruby_securecookie_http_allowed
