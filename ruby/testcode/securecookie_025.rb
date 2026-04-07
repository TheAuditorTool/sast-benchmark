require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_securecookie_clear_cookie_token
def set_api_token_cookie(req)
  api_token = SecureRandom.hex(32)
  response = BenchmarkResponse.ok('token set')
  response.headers['Set-Cookie'] = "api_token=#{api_token}; Secure; SameSite=Strict" # vuln-code-snippet vuln-line ruby_securecookie_clear_cookie_token
  response
end
# vuln-code-snippet end ruby_securecookie_clear_cookie_token
