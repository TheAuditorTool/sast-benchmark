require_relative 'shared'
require 'openssl'
require 'securerandom'

def store_password_pbkdf2(req)
  password = req.param('password')
  salt   = SecureRandom.random_bytes(16)
  hashed = OpenSSL::PKCS5.pbkdf2_hmac(password, salt, 600_000, 32, 'SHA256')
  BenchmarkResponse.json({ hash: hashed.unpack1('H*'), salt: salt.unpack1('H*') })
end
