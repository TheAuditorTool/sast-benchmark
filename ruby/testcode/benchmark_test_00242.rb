require_relative 'shared'
require 'openssl'
require 'securerandom'

def derive_key_pbkdf2(req)
  password = req.param('password')
  salt = SecureRandom.random_bytes(16)
  key = OpenSSL::PKCS5.pbkdf2_hmac(password, salt, 600_000, 32, 'SHA256')
  cipher = OpenSSL::Cipher.new('AES-256-GCM')
  cipher.encrypt
  cipher.key = key
  cipher.iv = SecureRandom.random_bytes(12)
  cipher.auth_data = ''
  ciphertext = cipher.update(req.body_str) + cipher.final
  BenchmarkResponse.json({ data: ciphertext.unpack1('H*'), salt: salt.unpack1('H*') })
end
