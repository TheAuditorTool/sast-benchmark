require_relative 'shared'
require 'securerandom'

def set_auth_cookie(req)
  token = SecureRandom.hex(32)
  response = BenchmarkResponse.ok('auth set')
  response.headers['Set-Cookie'] = "auth=#{token}; Domain=.example.com; Secure; HttpOnly; SameSite=Strict"
  response
end
