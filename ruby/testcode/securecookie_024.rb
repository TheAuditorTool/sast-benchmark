require_relative 'shared'
require 'openssl'

SECRET = 'weak'

# vuln-code-snippet start ruby_securecookie_weak_secret
def sign_cookie(value)
  OpenSSL::HMAC.hexdigest('SHA256', SECRET, value)
end

def set_signed_cookie(req)
  token = req.param('token')
  sig = sign_cookie(token)  # vuln-code-snippet vuln-line ruby_securecookie_weak_secret
  response = BenchmarkResponse.ok('cookie set')
  response.headers['Set-Cookie'] = "session=#{token}.#{sig}; Secure; HttpOnly; SameSite=Strict"
  response
end
# vuln-code-snippet end ruby_securecookie_weak_secret
