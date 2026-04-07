require_relative 'shared'
require 'securerandom'

def generate_token(req)
  token = SecureRandom.random_number(2**128)
  BenchmarkResponse.json({ token: token })
end
