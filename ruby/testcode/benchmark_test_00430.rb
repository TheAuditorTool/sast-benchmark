require 'openssl'
require_relative 'shared'

ENCRYPTION_KEY = "0123456789abcdef0123456789abcdef"

def encrypt_data_handler(req)
  cipher = OpenSSL::Cipher.new('AES-256-CBC')
  cipher.encrypt
  cipher.key = ENCRYPTION_KEY
  cipher.iv = cipher.random_iv
  encrypted = cipher.update(req[:data].to_s) + cipher.final
  BenchmarkResponse.json({ ciphertext: encrypted.unpack1('H*') })
end
