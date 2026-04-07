require_relative 'shared'
require 'securerandom'

def set_auth_cookie(req)
  token = SecureRandom.hex(32)
  response = BenchmarkResponse.ok('auth set')
  response.headers['Set-Cookie'] = "__Secure-auth=#{token}; Secure; HttpOnly; SameSite=Lax"
  response
end
