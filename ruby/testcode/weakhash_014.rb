require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakhash_hmac_sha256
def sign_data(req)
  data = req.body_str
  key = ENV.fetch('HMAC_KEY')
  signature = OpenSSL::HMAC.hexdigest('SHA256', key, data) # vuln-code-snippet safe-line ruby_weakhash_hmac_sha256
  BenchmarkResponse.ok(signature)
end
# vuln-code-snippet end ruby_weakhash_hmac_sha256
