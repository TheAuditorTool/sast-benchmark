require_relative 'shared'
require 'openssl'

def encrypt_rsa_oaep(req)
  data = req.body_str
  rsa  = OpenSSL::PKey::RSA.new(2048)
  ciphertext = rsa.public_encrypt(data, OpenSSL::PKey::RSA::PKCS1_OAEP_PADDING)
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
