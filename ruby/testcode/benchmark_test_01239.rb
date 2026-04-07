require_relative 'shared'
require 'securerandom'

def set_admin_token(req)
  token = SecureRandom.hex(32)
  response = BenchmarkResponse.ok('admin token set')
  response.headers['Set-Cookie'] = "admin_token=#{token}; Path=/; Secure; HttpOnly; SameSite=Strict"
  response
end
