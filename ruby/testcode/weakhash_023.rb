require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakhash_hmac_sha1_password
def store_password_hmac_sha1(req)
  password = req.param('password')
  secret   = ENV.fetch('APP_SECRET', 'default_secret')
  digest = OpenSSL::HMAC.hexdigest('SHA1', secret, password) # vuln-code-snippet vuln-line ruby_weakhash_hmac_sha1_password
  BenchmarkResponse.json({ hash: digest })
end
# vuln-code-snippet end ruby_weakhash_hmac_sha1_password
