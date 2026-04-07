require_relative 'shared'
require 'securerandom'

def generate_token(req)
  token = SecureRandom.urlsafe_base64(32)
  BenchmarkResponse.json({ token: token })
end
