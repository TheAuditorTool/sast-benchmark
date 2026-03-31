require_relative 'shared'
require 'digest'

# vuln-code-snippet start ruby_weakhash_sha1
def generate_auth_token(req)
  password = req.post('password')
  token = Digest::SHA1.hexdigest(password) # vuln-code-snippet vuln-line ruby_weakhash_sha1
  BenchmarkResponse.ok(token)
end
# vuln-code-snippet end ruby_weakhash_sha1
