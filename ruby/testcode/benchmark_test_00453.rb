require_relative 'shared'
require 'openssl'

def encrypt_password_key(req)
  plaintext = req.body_str
  password = req.param('password')
  key = password.ljust(32, "\x00")[0, 32]
  cipher = OpenSSL::Cipher.new('AES-256-CBC')
  cipher.encrypt
  cipher.key = key
  cipher.iv = cipher.random_iv
  encrypted = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(encrypted.unpack1('H*'))
end
