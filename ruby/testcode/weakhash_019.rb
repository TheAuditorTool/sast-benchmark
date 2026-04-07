require_relative 'shared'
require 'digest'

# vuln-code-snippet start ruby_weakhash_sha256_fast_password
def store_password_sha256(req)
  password = req.param('password')
  digest = Digest::SHA256.hexdigest(password) # vuln-code-snippet vuln-line ruby_weakhash_sha256_fast_password
  BenchmarkResponse.json({ hash: digest })
end
# vuln-code-snippet end ruby_weakhash_sha256_fast_password
