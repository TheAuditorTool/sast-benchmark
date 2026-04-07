require_relative 'shared'
require 'openssl'

def hash_data_sha3(req)
  data   = req.body_str
  digest = OpenSSL::Digest.new('SHA3-256').hexdigest(data)
  BenchmarkResponse.json({ digest: digest })
end
