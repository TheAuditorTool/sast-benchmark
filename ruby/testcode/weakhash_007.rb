require_relative 'shared'
require 'digest'

# vuln-code-snippet start ruby_weakhash_sha256_nosalt
def store_password(req)
  password = req.post('password')
  hashed = Digest::SHA256.hexdigest(password) # vuln-code-snippet vuln-line ruby_weakhash_sha256_nosalt
  BenchmarkResponse.ok(hashed)
end
# vuln-code-snippet end ruby_weakhash_sha256_nosalt
