require_relative 'shared'
require 'openssl'

STATIC_KEY = 'ThisIsASecretKey1234567890123456'

# vuln-code-snippet start ruby_weakcipher_static_key
def encrypt_static_key(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('AES-256-CBC')
  cipher.encrypt
  cipher.key = STATIC_KEY # vuln-code-snippet vuln-line ruby_weakcipher_static_key
  cipher.iv = cipher.random_iv
  encrypted = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(encrypted.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakcipher_static_key
