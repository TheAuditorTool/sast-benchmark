require_relative 'shared'
require 'openssl'

SECRET = 'weak'

def sign_cookie(value)
  OpenSSL::HMAC.hexdigest('SHA256', SECRET, value)
end

def set_signed_cookie(req)
  token = req.param('token')
  sig = sign_cookie(token)
  response = BenchmarkResponse.ok('cookie set')
  response.headers['Set-Cookie'] = "session=#{token}.#{sig}; Secure; HttpOnly; SameSite=Strict"
  response
end
