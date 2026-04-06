require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_authn_secure_remember
def create_remember_safe(req)
  user_id = req.param('user_id')
  token = SecureRandom.hex(32) # vuln-code-snippet safe-line ruby_authn_secure_remember
  BenchmarkResponse.ok("remember: #{token}")
end
# vuln-code-snippet end ruby_authn_secure_remember
