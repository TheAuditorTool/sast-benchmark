require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakhash_openssl_sha1
def hash_password_sha1(req)
  password = req.param('password')
  hashed = OpenSSL::Digest.new('SHA1').hexdigest(password) # vuln-code-snippet vuln-line ruby_weakhash_openssl_sha1
  BenchmarkResponse.ok(hashed)
end
# vuln-code-snippet end ruby_weakhash_openssl_sha1
