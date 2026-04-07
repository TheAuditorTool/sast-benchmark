require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakhash_sha256_hmac_api_sign
def sign_api_payload(req)
  payload    = req.body_str
  api_secret = ENV.fetch('API_SECRET')
  signature = OpenSSL::HMAC.hexdigest('SHA256', api_secret, payload) # vuln-code-snippet safe-line ruby_weakhash_sha256_hmac_api_sign
  BenchmarkResponse.json({ signature: signature })
end
# vuln-code-snippet end ruby_weakhash_sha256_hmac_api_sign
