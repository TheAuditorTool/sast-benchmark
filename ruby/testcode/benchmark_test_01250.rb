require_relative 'shared'
require 'securerandom'

def generate_session_token_safe(req)
  _user = req.param('user')
  token = SecureRandom.hex(32)
  BenchmarkResponse.ok(token)
end
