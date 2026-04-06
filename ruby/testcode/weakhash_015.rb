require_relative 'shared'
require 'digest'

# vuln-code-snippet start ruby_weakhash_sha256_plain
def hash_password_nosalt(req)
  password = req.param('password')
  hashed = Digest::SHA256.hexdigest(password) # vuln-code-snippet vuln-line ruby_weakhash_sha256_plain
  BenchmarkResponse.ok(hashed)
end
# vuln-code-snippet end ruby_weakhash_sha256_plain
