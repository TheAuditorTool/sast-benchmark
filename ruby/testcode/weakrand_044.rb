require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakrand_openssl_bn_rand
def generate_token(req)
  n = OpenSSL::BN.rand(128) # vuln-code-snippet safe-line ruby_weakrand_openssl_bn_rand
  BenchmarkResponse.json({ token: n.to_s(16) })
end
# vuln-code-snippet end ruby_weakrand_openssl_bn_rand
