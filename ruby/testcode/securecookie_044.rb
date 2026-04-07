require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_securecookie_api_no_cookie
def issue_token(req)
  jwt_token = SecureRandom.hex(32) # vuln-code-snippet safe-line ruby_securecookie_api_no_cookie
  BenchmarkResponse.json({ token: jwt_token })
end
# vuln-code-snippet end ruby_securecookie_api_no_cookie
