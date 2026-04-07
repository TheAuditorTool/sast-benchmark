require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_securecookie_subdomain_wide
def set_auth_cookie(req)
  token = SecureRandom.hex(32)
  response = BenchmarkResponse.ok('auth set')
  response.headers['Set-Cookie'] = "auth=#{token}; Domain=.example.com; Secure; HttpOnly; SameSite=Strict" # vuln-code-snippet vuln-line ruby_securecookie_subdomain_wide
  response
end
# vuln-code-snippet end ruby_securecookie_subdomain_wide
