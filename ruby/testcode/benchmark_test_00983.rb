require_relative 'shared'

begin
  require 'rbnacl'
rescue LoadError
end

def encrypt_secretbox(req)
  msg   = req.body_str
  key   = RbNaCl::Random.random_bytes(RbNaCl::SecretBox.key_bytes)
  box   = RbNaCl::SecretBox.new(key)
  nonce = RbNaCl::Random.random_bytes(box.nonce_bytes)
  ciphertext = box.box(nonce, msg)
  BenchmarkResponse.json({ data: ciphertext.unpack1('H*'), nonce: nonce.unpack1('H*') })
end
