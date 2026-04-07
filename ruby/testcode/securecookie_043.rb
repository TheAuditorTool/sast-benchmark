require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_securecookie_csrf_cookie
def set_csrf_cookie(req)
  token = SecureRandom.hex(32)
  response = BenchmarkResponse.ok('csrf set')
  response.headers['Set-Cookie'] = "csrf_token=#{token}; Secure; SameSite=Strict" # vuln-code-snippet safe-line ruby_securecookie_csrf_cookie
  response
end
# vuln-code-snippet end ruby_securecookie_csrf_cookie
