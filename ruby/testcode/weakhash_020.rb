require_relative 'shared'
require 'digest'

# vuln-code-snippet start ruby_weakhash_sha512_fast
def store_password_sha512(req)
  password = req.param('password')
  digest = Digest::SHA512.hexdigest(password) # vuln-code-snippet vuln-line ruby_weakhash_sha512_fast
  BenchmarkResponse.json({ hash: digest })
end
# vuln-code-snippet end ruby_weakhash_sha512_fast
