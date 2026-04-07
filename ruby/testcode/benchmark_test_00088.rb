require_relative 'shared'
require 'digest'

def sign_api_request(req)
  payload    = req.body_str
  api_secret = ENV.fetch('API_SECRET', 'default')
  signature = Digest::MD5.hexdigest(payload + api_secret)
  BenchmarkResponse.json({ signature: signature })
end
