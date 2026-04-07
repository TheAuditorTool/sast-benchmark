require_relative 'shared'
require 'openssl'

def encrypt_des_ecb(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('DES-ECB')
  cipher.encrypt
  cipher.key = cipher.random_key
  ciphertext = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
