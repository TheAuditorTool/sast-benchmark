require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakrand_openssl_random
def generate_key_material(req)
  key = OpenSSL::Random.random_bytes(32) # vuln-code-snippet safe-line ruby_weakrand_openssl_random
  BenchmarkResponse.ok(key.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakrand_openssl_random
