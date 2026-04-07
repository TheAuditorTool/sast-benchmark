require_relative 'shared'
require 'digest'

# vuln-code-snippet start ruby_weakhash_sha256_time_seed
def store_password_time_salt(req)
  password = req.param('password')
  digest = Digest::SHA256.hexdigest(password + Time.now.to_s) # vuln-code-snippet vuln-line ruby_weakhash_sha256_time_seed
  BenchmarkResponse.json({ hash: digest })
end
# vuln-code-snippet end ruby_weakhash_sha256_time_seed
