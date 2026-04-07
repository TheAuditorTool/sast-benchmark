require_relative 'shared'
require 'digest'

STATIC_SALT = 'deadbeefdeadbeef'

# vuln-code-snippet start ruby_weakhash_sha256_concat_salt
def store_password_static_salt(req)
  password = req.param('password')
  digest = Digest::SHA256.hexdigest(STATIC_SALT + password) # vuln-code-snippet vuln-line ruby_weakhash_sha256_concat_salt
  BenchmarkResponse.json({ hash: digest })
end
# vuln-code-snippet end ruby_weakhash_sha256_concat_salt
