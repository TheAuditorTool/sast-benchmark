require_relative 'shared'
require 'openssl'

def derive_key(req)
  password = req.post('password')
  salt = SecureRandom.random_bytes(16)
  key = OpenSSL::PKCS5.pbkdf2_hmac(password, salt, 100_000, 32, 'SHA256')
  BenchmarkResponse.ok(key.unpack1('H*'))
end
