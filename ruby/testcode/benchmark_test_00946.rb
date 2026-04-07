require_relative 'shared'
require 'openssl'

def sign_payload(req)
  data   = req.body_str
  secret = ENV.fetch('HMAC_SECRET')
  mac = OpenSSL::HMAC.hexdigest('SHA256', secret, data)
  BenchmarkResponse.json({ mac: mac })
end
