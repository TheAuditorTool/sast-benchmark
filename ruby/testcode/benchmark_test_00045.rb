require_relative 'shared'
require 'securerandom'

PRODUCTION = false

def set_auth_cookie(req)
  token = SecureRandom.hex(32)
  secure_flag = PRODUCTION ? 'Secure; ' : ''
  response = BenchmarkResponse.ok('auth set')
  response.headers['Set-Cookie'] = "auth=#{token}; #{secure_flag}HttpOnly; SameSite=Strict"
  response
end
