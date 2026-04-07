require_relative 'shared'

begin
  require 'rbnacl'
rescue LoadError
end

def encrypt_nacl_box(req)
  msg          = req.body_str
  sender_sk    = RbNaCl::PrivateKey.generate
  recipient_pk = RbNaCl::PrivateKey.generate.public_key
  box          = RbNaCl::Box.new(recipient_pk, sender_sk)
  nonce        = RbNaCl::Random.random_bytes(box.nonce_bytes)
  ciphertext   = box.box(nonce, msg)
  BenchmarkResponse.json({ data: ciphertext.unpack1('H*'), nonce: nonce.unpack1('H*') })
end
