require_relative 'shared'
require 'securerandom'

def generate_token(req)
  token = SecureRandom.alphanumeric(32)
  BenchmarkResponse.json({ token: token })
end
