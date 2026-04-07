require_relative 'shared'

begin
  require 'rbnacl'
rescue LoadError
end

def mac_poly1305(req)
  msg = req.body_str
  key = RbNaCl::Random.random_bytes(RbNaCl::OneTimeAuth.key_bytes)
  mac = RbNaCl::OneTimeAuth.auth(key, msg)
  BenchmarkResponse.json({ mac: mac.unpack1('H*') })
end
