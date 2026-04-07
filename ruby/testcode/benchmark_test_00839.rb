require_relative 'shared'
require 'openssl'

def hash_token(req)
  token = req.param('token')
  hashed = OpenSSL::Digest.new('MD5').hexdigest(token)
  BenchmarkResponse.ok(hashed)
end
