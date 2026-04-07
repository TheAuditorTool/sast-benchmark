require_relative 'shared'
require 'securerandom'

def set_persistent_auth(req)
  token = SecureRandom.hex(32)
  expires = (Time.now + 365 * 24 * 3600).httpdate
  response = BenchmarkResponse.ok('auth set')
  response.headers['Set-Cookie'] = "auth=#{token}; Secure; HttpOnly; SameSite=Strict; Expires=#{expires}"
  response
end
