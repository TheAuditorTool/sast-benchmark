require_relative 'shared'
require 'securerandom'

def create_remember_safe(req)
  user_id = req.param('user_id')
  token = SecureRandom.hex(32)
  BenchmarkResponse.ok("remember: #{token}")
end
