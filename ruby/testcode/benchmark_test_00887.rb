require_relative 'shared'
require 'openssl'
require 'securerandom'

def encrypt_chacha20(req)
  plaintext = req.body_str
  key = OpenSSL::Random.random_bytes(32)
  nonce = SecureRandom.random_bytes(12)
  cipher = OpenSSL::Cipher.new('chacha20-poly1305')
  cipher.encrypt
  cipher.key = key
  cipher.iv = nonce
  cipher.auth_data = ''
  ciphertext = cipher.update(plaintext) + cipher.final
  tag = cipher.auth_tag
  BenchmarkResponse.json({ data: ciphertext.unpack1('H*'), tag: tag.unpack1('H*') })
end
