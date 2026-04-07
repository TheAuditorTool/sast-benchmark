require_relative 'shared'
require 'openssl'

def sign_api_payload(req)
  payload    = req.body_str
  api_secret = ENV.fetch('API_SECRET')
  signature = OpenSSL::HMAC.hexdigest('SHA256', api_secret, payload)
  BenchmarkResponse.json({ signature: signature })
end
