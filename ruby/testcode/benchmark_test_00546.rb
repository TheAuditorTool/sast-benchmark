require_relative 'shared'
require 'openssl'

def encrypt_blowfish(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('BF-CBC')
  cipher.encrypt
  cipher.key = 'hardcoded_blowfish_key_16bytes!!'
  cipher.iv = cipher.random_iv
  ciphertext = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
