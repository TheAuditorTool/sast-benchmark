require_relative 'shared'
require 'securerandom'

def generate_iv(req)
  nonce = SecureRandom.random_bytes(12)
  BenchmarkResponse.json({ nonce: nonce.unpack1('H*') })
end
