require_relative 'shared'

begin
  require 'rbnacl'
rescue LoadError
  # rbnacl gem not available -- syntax-checked only
end

# vuln-code-snippet start ruby_weakcipher_nacl_onetimeauth
def mac_poly1305(req)
  msg = req.body_str
  key = RbNaCl::Random.random_bytes(RbNaCl::OneTimeAuth.key_bytes)
  mac = RbNaCl::OneTimeAuth.auth(key, msg) # vuln-code-snippet safe-line ruby_weakcipher_nacl_onetimeauth
  BenchmarkResponse.json({ mac: mac.unpack1('H*') })
end
# vuln-code-snippet end ruby_weakcipher_nacl_onetimeauth
