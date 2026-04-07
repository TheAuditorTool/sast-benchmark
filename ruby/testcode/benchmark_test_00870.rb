require_relative 'shared'
require 'openssl'

def generate_key_material(req)
  key = OpenSSL::Random.random_bytes(32)
  BenchmarkResponse.ok(key.unpack1('H*'))
end
