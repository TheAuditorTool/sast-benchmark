require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakcipher_truncated_key
def encrypt_password_key(req)
  plaintext = req.body_str
  password = req.param('password')
  key = password.ljust(32, "\x00")[0, 32] # vuln-code-snippet vuln-line ruby_weakcipher_truncated_key
  cipher = OpenSSL::Cipher.new('AES-256-CBC')
  cipher.encrypt
  cipher.key = key
  cipher.iv = cipher.random_iv
  encrypted = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(encrypted.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakcipher_truncated_key
