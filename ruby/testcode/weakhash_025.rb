require_relative 'shared'
require 'digest'

# vuln-code-snippet start ruby_weakhash_sha256_truncated
def store_password_truncated(req)
  password = req.param('password')
  digest = Digest::SHA256.hexdigest(password)[0..15] # vuln-code-snippet vuln-line ruby_weakhash_sha256_truncated
  BenchmarkResponse.json({ hash: digest })
end
# vuln-code-snippet end ruby_weakhash_sha256_truncated
