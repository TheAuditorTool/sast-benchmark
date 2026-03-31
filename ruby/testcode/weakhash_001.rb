require_relative 'shared'
require 'digest'

# vuln-code-snippet start ruby_weakhash_md5
def hash_password(req)
  password = req.post('password')
  hashed = Digest::MD5.hexdigest(password) # vuln-code-snippet vuln-line ruby_weakhash_md5
  BenchmarkResponse.ok(hashed)
end
# vuln-code-snippet end ruby_weakhash_md5
