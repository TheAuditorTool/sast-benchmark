require_relative 'shared'
require 'securerandom'

def set_payment_session(req)
  token = SecureRandom.hex(32)
  response = BenchmarkResponse.ok('payment session set')
  response.headers['Set-Cookie'] = "payment_session=#{token}; Secure; HttpOnly; SameSite=Lax"
  response
end
