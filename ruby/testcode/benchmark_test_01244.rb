require_relative 'shared'
require 'securerandom'

def generate_key(req)
  token = SecureRandom.random_bytes(32)
  BenchmarkResponse.json({ key: token.unpack1('H*') })
end
