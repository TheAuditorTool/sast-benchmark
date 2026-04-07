require_relative 'shared'
require 'openssl'

def generate_token(req)
  n = OpenSSL::BN.rand(128)
  BenchmarkResponse.json({ token: n.to_s(16) })
end
