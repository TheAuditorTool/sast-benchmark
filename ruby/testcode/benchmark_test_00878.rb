require_relative 'shared'
require 'securerandom'

def generate_token_hex(req)
  token = SecureRandom.hex(32)
  BenchmarkResponse.ok(token)
end
