require_relative 'shared'
require 'openssl'

def sign_data(req)
  key = ENV.fetch('HMAC_SECRET', 'default_key')
  data = req.body_str
  mac = OpenSSL::HMAC.hexdigest('SHA256', key, data)
  BenchmarkResponse.ok(mac)
end
