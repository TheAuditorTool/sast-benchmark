require_relative 'shared'
require 'securerandom'

def generate_uuid(req)
  id = SecureRandom.uuid
  BenchmarkResponse.ok(id)
end
