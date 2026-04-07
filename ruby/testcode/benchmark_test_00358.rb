require_relative 'shared'
require 'securerandom'

def set_session(req)
  token = SecureRandom.hex(32)
  expires = (Time.now + 900).httpdate
  response = BenchmarkResponse.ok('session set')
  response.headers['Set-Cookie'] = "session=#{token}; Secure; HttpOnly; SameSite=Strict; Expires=#{expires}"
  response
end
