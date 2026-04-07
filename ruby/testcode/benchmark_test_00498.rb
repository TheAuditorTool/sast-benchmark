require_relative 'shared'
require 'securerandom'

def create_session(req)
  session_token = SecureRandom.hex(32)
  BenchmarkResponse.json({ session_token: session_token })
end
