require_relative 'shared'

begin
  require 'rbnacl'
rescue LoadError
  # rbnacl gem not available -- syntax-checked only
end

# vuln-code-snippet start ruby_weakrand_rbnacl_random
def generate_token(req)
  token = RbNaCl::Random.random_bytes(32) # vuln-code-snippet safe-line ruby_weakrand_rbnacl_random
  BenchmarkResponse.json({ token: token.unpack1('H*') })
end
# vuln-code-snippet end ruby_weakrand_rbnacl_random
