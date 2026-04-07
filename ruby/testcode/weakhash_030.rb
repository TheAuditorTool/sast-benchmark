require_relative 'shared'
require 'digest'

# vuln-code-snippet start ruby_weakhash_md5_api_sign
def sign_api_request(req)
  payload    = req.body_str
  api_secret = ENV.fetch('API_SECRET', 'default')
  signature = Digest::MD5.hexdigest(payload + api_secret) # vuln-code-snippet vuln-line ruby_weakhash_md5_api_sign
  BenchmarkResponse.json({ signature: signature })
end
# vuln-code-snippet end ruby_weakhash_md5_api_sign
