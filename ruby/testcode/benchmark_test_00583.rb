require_relative 'shared'
require 'securerandom'

def set_remember_token(req)
  token = SecureRandom.hex(32)
  expires = (Time.now + 365 * 24 * 3600).httpdate
  response = BenchmarkResponse.ok('remember set')
  response.headers['Set-Cookie'] = "remember_token=#{token}; Secure; HttpOnly; SameSite=Lax; Expires=#{expires}"
  response
end
