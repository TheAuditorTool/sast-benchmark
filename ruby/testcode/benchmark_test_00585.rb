require_relative 'shared'

begin
  require 'rbnacl'
rescue LoadError
end

def hash_data_blake2b(req)
  data   = req.body_str
  digest = RbNaCl::Hash.blake2b(data)
  BenchmarkResponse.json({ digest: digest.unpack1('H*') })
end
