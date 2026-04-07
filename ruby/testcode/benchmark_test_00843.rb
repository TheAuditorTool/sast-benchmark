require_relative 'shared'
require 'securerandom'

def set_csrf_cookie(req)
  token = SecureRandom.hex(32)
  response = BenchmarkResponse.ok('csrf set')
  response.headers['Set-Cookie'] = "csrf_token=#{token}; Secure; SameSite=Strict"
  response
end
