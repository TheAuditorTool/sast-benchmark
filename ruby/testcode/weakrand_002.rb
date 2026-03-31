require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_weakrand_securerandom
def generate_session_token_safe(req)
  _user = req.param('user')
  token = SecureRandom.hex(32) # vuln-code-snippet safe-line ruby_weakrand_securerandom
  BenchmarkResponse.ok(token)
end
# vuln-code-snippet end ruby_weakrand_securerandom
