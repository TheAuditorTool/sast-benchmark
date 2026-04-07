require_relative 'shared'
require 'openssl'

def encrypt_null_iv(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('AES-256-CBC')
  cipher.encrypt
  cipher.key = cipher.random_key
  cipher.iv = "\x00" * 16
  encrypted = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(encrypted.unpack1('H*'))
end
