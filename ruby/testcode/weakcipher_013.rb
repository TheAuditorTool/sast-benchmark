require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakcipher_null_iv
def encrypt_null_iv(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('AES-256-CBC')
  cipher.encrypt
  cipher.key = cipher.random_key
  cipher.iv = "\x00" * 16 # vuln-code-snippet vuln-line ruby_weakcipher_null_iv
  encrypted = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(encrypted.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakcipher_null_iv
