require_relative 'shared'
require 'openssl'

def encrypt_3des(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('DES-EDE3-CBC')
  cipher.encrypt
  cipher.key = cipher.random_key
  cipher.iv = cipher.random_iv
  encrypted = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(encrypted.unpack1('H*'))
end
