require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_weakrand_nonce_csprng
def generate_iv(req)
  nonce = SecureRandom.random_bytes(12) # vuln-code-snippet safe-line ruby_weakrand_nonce_csprng
  BenchmarkResponse.json({ nonce: nonce.unpack1('H*') })
end
# vuln-code-snippet end ruby_weakrand_nonce_csprng
