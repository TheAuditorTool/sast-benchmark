require_relative 'shared'
require 'securerandom'

def set_session_rodauth(req)
  token = SecureRandom.hex(32)
  response = BenchmarkResponse.ok('rodauth session set')
  response.headers['Set-Cookie'] = "session=#{token}; Secure; HttpOnly; SameSite=Strict; Path=/"
  response
end
