require_relative 'shared'
require 'securerandom'

def authenticate(req)
  token = SecureRandom.hex(32)
  response = BenchmarkResponse.ok('authenticated')
  response.headers['Set-Cookie'] = "session=#{token}; Secure; HttpOnly; SameSite=Strict; Path=/"
  response
end
