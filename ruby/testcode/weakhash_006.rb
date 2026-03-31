require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakhash_sha256_hmac
def sign_data(req)
  key = ENV.fetch('HMAC_SECRET', 'default_key')
  data = req.body_str
  mac = OpenSSL::HMAC.hexdigest('SHA256', key, data) # vuln-code-snippet safe-line ruby_weakhash_sha256_hmac
  BenchmarkResponse.ok(mac)
end
# vuln-code-snippet end ruby_weakhash_sha256_hmac
