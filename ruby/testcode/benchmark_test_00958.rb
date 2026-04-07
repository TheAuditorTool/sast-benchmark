require_relative 'shared'
require 'securerandom'

def generate_reset_token_weak(req)
  username = req.post('username')
  token = SecureRandom.hex(3)
  BenchmarkResponse.ok("Reset token for #{username}: #{token}")
end
