require_relative 'shared'
require 'openssl'

def generate_token(req)
  token = OpenSSL::Random.random_bytes(32)
  BenchmarkResponse.json({ token: token.unpack1('H*') })
end
