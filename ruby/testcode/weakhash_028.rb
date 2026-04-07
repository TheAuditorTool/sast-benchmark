require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakhash_openssl_sha256_nosalt
def store_password_openssl_sha256(req)
  password = req.param('password')
  digest = OpenSSL::Digest.new('SHA256').hexdigest(password) # vuln-code-snippet vuln-line ruby_weakhash_openssl_sha256_nosalt
  BenchmarkResponse.json({ hash: digest })
end
# vuln-code-snippet end ruby_weakhash_openssl_sha256_nosalt
