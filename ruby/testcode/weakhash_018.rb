require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakhash_pbkdf2_derive
def hash_password_pbkdf2(req)
  password = req.param('password')
  salt = OpenSSL::Random.random_bytes(16)
  hashed = OpenSSL::PKCS5.pbkdf2_hmac(password, salt, 100_000, 32, 'SHA256') # vuln-code-snippet safe-line ruby_weakhash_pbkdf2_derive
  BenchmarkResponse.ok(hashed.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakhash_pbkdf2_derive
