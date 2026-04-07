require_relative 'shared'
require 'openssl'

STATIC_KEY = 'ThisIsASecretKey1234567890123456'

def encrypt_static_key(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('AES-256-CBC')
  cipher.encrypt
  cipher.key = STATIC_KEY
  cipher.iv = cipher.random_iv
  encrypted = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(encrypted.unpack1('H*'))
end
