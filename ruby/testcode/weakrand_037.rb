require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakrand_openssl_random_bytes
def generate_token(req)
  token = OpenSSL::Random.random_bytes(32) # vuln-code-snippet safe-line ruby_weakrand_openssl_random_bytes
  BenchmarkResponse.json({ token: token.unpack1('H*') })
end
# vuln-code-snippet end ruby_weakrand_openssl_random_bytes
