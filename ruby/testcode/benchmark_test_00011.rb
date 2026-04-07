require_relative 'shared'
require 'digest'

def generate_token(req)
  token = Digest::SHA256.hexdigest(rand.to_s)
  BenchmarkResponse.json({ token: token })
end
