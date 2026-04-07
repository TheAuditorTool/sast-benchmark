require_relative 'shared'
require 'digest'

def hash_legacy(req)
  data = req.body_str
  hashed = OpenSSL::Digest.new('MD5').hexdigest(data)
  BenchmarkResponse.ok(hashed)
end
