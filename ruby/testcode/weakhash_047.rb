require_relative 'shared'

begin
  require 'rbnacl'
rescue LoadError
  # rbnacl gem not available -- syntax-checked only
end

# vuln-code-snippet start ruby_weakhash_blake2b_integrity
def hash_data_blake2b(req)
  data   = req.body_str
  digest = RbNaCl::Hash.blake2b(data) # vuln-code-snippet safe-line ruby_weakhash_blake2b_integrity
  BenchmarkResponse.json({ digest: digest.unpack1('H*') })
end
# vuln-code-snippet end ruby_weakhash_blake2b_integrity
