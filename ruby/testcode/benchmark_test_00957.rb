require_relative 'shared'
require 'securerandom'

def provision_api_key(req)
  api_key = SecureRandom.hex(32)
  BenchmarkResponse.json({ api_key: api_key })
end
