require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_weakrand_securerandom_hex
def generate_token_hex(req)
  token = SecureRandom.hex(32) # vuln-code-snippet safe-line ruby_weakrand_securerandom_hex
  BenchmarkResponse.ok(token)
end
# vuln-code-snippet end ruby_weakrand_securerandom_hex
