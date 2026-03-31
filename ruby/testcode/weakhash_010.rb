require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakhash_pbkdf2
def derive_key(req)
  password = req.post('password')
  salt = SecureRandom.random_bytes(16)
  key = OpenSSL::PKCS5.pbkdf2_hmac(password, salt, 100_000, 32, 'SHA256') # vuln-code-snippet safe-line ruby_weakhash_pbkdf2
  BenchmarkResponse.ok(key.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakhash_pbkdf2
