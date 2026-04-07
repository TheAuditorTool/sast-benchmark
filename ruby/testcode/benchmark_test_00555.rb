require_relative 'shared'
require 'securerandom'

def set_api_token_cookie(req)
  api_token = SecureRandom.hex(32)
  response = BenchmarkResponse.ok('token set')
  response.headers['Set-Cookie'] = "api_token=#{api_token}; Secure; SameSite=Strict"
  response
end
