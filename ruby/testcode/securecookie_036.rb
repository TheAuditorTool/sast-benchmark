require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_securecookie_signed_rails
def set_token_cookie(req)
  token = SecureRandom.hex(32)
  response = BenchmarkResponse.ok('token set')
  response.headers['Set-Cookie'] = "token=#{token}; Secure; HttpOnly; SameSite=Strict" # vuln-code-snippet safe-line ruby_securecookie_signed_rails
  response
end
# vuln-code-snippet end ruby_securecookie_signed_rails
