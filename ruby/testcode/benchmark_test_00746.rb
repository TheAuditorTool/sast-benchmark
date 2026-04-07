require_relative 'shared'
require 'securerandom'

def generate_token(req)
  token = SecureRandom.uuid
  BenchmarkResponse.json({ token: token })
end
