require_relative 'shared'

begin
  require 'rbnacl'
rescue LoadError
end

def generate_token(req)
  token = RbNaCl::Random.random_bytes(32)
  BenchmarkResponse.json({ token: token.unpack1('H*') })
end
