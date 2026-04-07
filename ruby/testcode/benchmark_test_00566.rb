require_relative 'shared'
require 'openssl'

def sign_data(req)
  data = req.body_str
  key = ENV.fetch('HMAC_KEY')
  signature = OpenSSL::HMAC.hexdigest('SHA256', key, data)
  BenchmarkResponse.ok(signature)
end
