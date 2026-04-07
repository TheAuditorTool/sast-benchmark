require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakhash_openssl_sha3_256
def hash_data_sha3(req)
  data   = req.body_str
  digest = OpenSSL::Digest.new('SHA3-256').hexdigest(data) # vuln-code-snippet safe-line ruby_weakhash_openssl_sha3_256
  BenchmarkResponse.json({ digest: digest })
end
# vuln-code-snippet end ruby_weakhash_openssl_sha3_256
