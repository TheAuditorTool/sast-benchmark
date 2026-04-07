require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_securecookie_js_accessible
def set_tracking_cookie(req)
  id = SecureRandom.hex(16)
  response = BenchmarkResponse.ok('tracking set')
  response.headers['Set-Cookie'] = "tracking=#{id}; Secure; SameSite=Strict" # vuln-code-snippet vuln-line ruby_securecookie_js_accessible
  response
end
# vuln-code-snippet end ruby_securecookie_js_accessible
