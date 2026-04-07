require_relative 'shared'
require 'securerandom'

def set_token_cookie(req)
  token = SecureRandom.hex(32)
  response = BenchmarkResponse.ok('token set')
  response.headers['Set-Cookie'] = "token=#{token}; Secure; HttpOnly; SameSite=Strict"
  response
end
