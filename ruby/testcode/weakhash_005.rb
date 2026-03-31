require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakhash_openssl_md5
def hash_token(req)
  token = req.param('token')
  hashed = OpenSSL::Digest.new('MD5').hexdigest(token) # vuln-code-snippet vuln-line ruby_weakhash_openssl_md5
  BenchmarkResponse.ok(hashed)
end
# vuln-code-snippet end ruby_weakhash_openssl_md5
