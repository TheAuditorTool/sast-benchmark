require_relative 'shared'
require 'openssl'

def encrypt_ecb_256(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('AES-256-ECB')
  cipher.encrypt
  cipher.key = cipher.random_key
  ciphertext = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
