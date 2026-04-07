require_relative 'shared'
require 'securerandom'

def build_rack_app
  token = SecureRandom.hex(32)
  response = BenchmarkResponse.ok('rack session set')
  response.headers['Set-Cookie'] = "rack.session=#{token}; Secure; HttpOnly; SameSite=Strict; Path=/"
  response
end
