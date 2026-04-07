require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_securecookie_none_no_secure
def set_cross_site_cookie(req)
  token = SecureRandom.hex(32)
  response = BenchmarkResponse.ok('cross-site cookie set')
  response.headers['Set-Cookie'] = "cross_site=#{token}; SameSite=None" # vuln-code-snippet vuln-line ruby_securecookie_none_no_secure
  response
end
# vuln-code-snippet end ruby_securecookie_none_no_secure
