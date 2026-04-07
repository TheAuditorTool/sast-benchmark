require_relative 'shared'
require 'securerandom'

def generate_code(req)
  code = SecureRandom.alphanumeric(32)
  BenchmarkResponse.ok(code)
end
