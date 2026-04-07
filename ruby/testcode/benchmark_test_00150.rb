require_relative 'shared'
require 'openssl'

def encrypt_rc2(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('RC2-CBC')
  cipher.encrypt
  cipher.key = cipher.random_key
  cipher.iv = cipher.random_iv
  encrypted = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(encrypted.unpack1('H*'))
end
