require_relative 'shared'
require 'securerandom'

def generate_token_base64(req)
  token = SecureRandom.urlsafe_base64(32)
  BenchmarkResponse.ok(token)
end
