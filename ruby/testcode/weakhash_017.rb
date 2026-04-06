require_relative 'shared'
require 'digest'

# vuln-code-snippet start ruby_weakhash_md5_legacy
def hash_legacy(req)
  data = req.body_str
  hashed = OpenSSL::Digest.new('MD5').hexdigest(data) # vuln-code-snippet vuln-line ruby_weakhash_md5_legacy
  BenchmarkResponse.ok(hashed)
end
# vuln-code-snippet end ruby_weakhash_md5_legacy
