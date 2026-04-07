require_relative 'shared'
require 'securerandom'

def set_tracking_cookie(req)
  id = SecureRandom.hex(16)
  response = BenchmarkResponse.ok('tracking set')
  response.headers['Set-Cookie'] = "tracking=#{id}; Secure; SameSite=Strict"
  response
end
