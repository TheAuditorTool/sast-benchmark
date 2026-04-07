require_relative 'shared'
require 'openssl'

def encrypt_blowfish(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('BF-CBC')
  cipher.encrypt
  cipher.key = cipher.random_key
  encrypted = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(encrypted.unpack1('H*'))
end
