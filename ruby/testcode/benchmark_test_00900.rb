require_relative 'shared'
require 'openssl'

def encrypt_srand_key(req)
  plaintext = req.body_str
  srand(42)
  key = (1..32).map { rand(256).chr }.join
  cipher = OpenSSL::Cipher.new('AES-256-CBC')
  cipher.encrypt
  cipher.key = key
  cipher.iv = cipher.random_iv
  encrypted = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(encrypted.unpack1('H*'))
end
