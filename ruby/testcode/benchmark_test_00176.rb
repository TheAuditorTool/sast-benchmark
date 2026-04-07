require_relative 'shared'
require 'openssl'
require 'securerandom'

def encrypt_cbc_env_key(req)
  plaintext = req.body_str
  key    = [ENV.fetch('ENCRYPTION_KEY')].pack('H*')
  iv     = SecureRandom.random_bytes(16)
  cipher = OpenSSL::Cipher.new('AES-256-CBC')
  cipher.encrypt
  cipher.key = key
  cipher.iv  = iv
  ciphertext = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.json({ iv: iv.unpack1('H*'), data: ciphertext.unpack1('H*') })
end
