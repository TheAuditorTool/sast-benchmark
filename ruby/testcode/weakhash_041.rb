require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakhash_hmac_sha256_integrity
def sign_payload(req)
  data   = req.body_str
  secret = ENV.fetch('HMAC_SECRET')
  mac = OpenSSL::HMAC.hexdigest('SHA256', secret, data) # vuln-code-snippet safe-line ruby_weakhash_hmac_sha256_integrity
  BenchmarkResponse.json({ mac: mac })
end
# vuln-code-snippet end ruby_weakhash_hmac_sha256_integrity
