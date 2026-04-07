require_relative 'shared'
require 'securerandom'

def issue_token(req)
  jwt_token = SecureRandom.hex(32)
  BenchmarkResponse.json({ token: jwt_token })
end
