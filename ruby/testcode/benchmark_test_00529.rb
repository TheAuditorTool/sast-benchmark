require_relative 'shared'
require 'openssl'
require 'securerandom'

def encrypt_aes256_gcm(req)
  plaintext = req.body_str
  key = OpenSSL::Random.random_bytes(32)
  cipher = OpenSSL::Cipher.new('AES-256-GCM')
  cipher.encrypt
  cipher.key = key
  cipher.iv = SecureRandom.random_bytes(12)
  cipher.auth_data = ''
  ciphertext = cipher.update(plaintext) + cipher.final
  tag = cipher.auth_tag
  BenchmarkResponse.json({ data: ciphertext.unpack1('H*'), tag: tag.unpack1('H*') })
end
