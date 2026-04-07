require_relative 'shared'
require 'openssl'
require 'securerandom'

# vuln-code-snippet start ruby_weakhash_pbkdf2_sha256_600k
def store_password_pbkdf2(req)
  password = req.param('password')
  salt   = SecureRandom.random_bytes(16)
  hashed = OpenSSL::PKCS5.pbkdf2_hmac(password, salt, 600_000, 32, 'SHA256') # vuln-code-snippet safe-line ruby_weakhash_pbkdf2_sha256_600k
  BenchmarkResponse.json({ hash: hashed.unpack1('H*'), salt: salt.unpack1('H*') })
end
# vuln-code-snippet end ruby_weakhash_pbkdf2_sha256_600k
