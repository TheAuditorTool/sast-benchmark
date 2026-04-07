require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_authn_short_token
def generate_reset_token_weak(req)
  username = req.post('username')
  token = SecureRandom.hex(3) # vuln-code-snippet vuln-line ruby_authn_short_token
  BenchmarkResponse.ok("Reset token for #{username}: #{token}")
end
# vuln-code-snippet end ruby_authn_short_token
