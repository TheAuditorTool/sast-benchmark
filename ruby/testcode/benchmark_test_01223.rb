require_relative 'shared'
require 'openssl'

def hash_password_pbkdf2(req)
  password = req.param('password')
  salt = OpenSSL::Random.random_bytes(16)
  hashed = OpenSSL::PKCS5.pbkdf2_hmac(password, salt, 100_000, 32, 'SHA256')
  BenchmarkResponse.ok(hashed.unpack1('H*'))
end
