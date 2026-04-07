require_relative 'shared'
require 'openssl'

def encrypt_3des_short(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('DES-EDE-CBC')
  cipher.encrypt
  cipher.key = cipher.random_key
  cipher.iv = cipher.random_iv
  ciphertext = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
