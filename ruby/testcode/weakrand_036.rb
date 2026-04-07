require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_weakrand_securerandom_bytes32
def generate_key(req)
  token = SecureRandom.random_bytes(32) # vuln-code-snippet safe-line ruby_weakrand_securerandom_bytes32
  BenchmarkResponse.json({ key: token.unpack1('H*') })
end
# vuln-code-snippet end ruby_weakrand_securerandom_bytes32
