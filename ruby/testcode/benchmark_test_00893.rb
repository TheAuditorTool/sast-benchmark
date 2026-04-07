require_relative 'shared'
require 'openssl'

def encrypt_aes256_cbc_proper(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('AES-256-CBC')
  cipher.encrypt
  key = cipher.random_key
  iv = cipher.random_iv
  encrypted = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok({ iv: iv.unpack1('H*'), data: encrypted.unpack1('H*') }.to_s)
end
