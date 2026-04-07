require_relative 'shared'

begin
  require 'rbnacl'
rescue LoadError
  # rbnacl gem not available -- syntax-checked only
end

# vuln-code-snippet start ruby_weakcipher_rbnacl_secretbox
def encrypt_secretbox(req)
  msg   = req.body_str
  key   = RbNaCl::Random.random_bytes(RbNaCl::SecretBox.key_bytes)
  box   = RbNaCl::SecretBox.new(key) # vuln-code-snippet safe-line ruby_weakcipher_rbnacl_secretbox
  nonce = RbNaCl::Random.random_bytes(box.nonce_bytes)
  ciphertext = box.box(nonce, msg)
  BenchmarkResponse.json({ data: ciphertext.unpack1('H*'), nonce: nonce.unpack1('H*') })
end
# vuln-code-snippet end ruby_weakcipher_rbnacl_secretbox
