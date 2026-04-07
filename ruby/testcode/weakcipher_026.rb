require_relative 'shared'
require 'openssl'

KEY = '0123456789abcdef0123456789abcdef'

# vuln-code-snippet start ruby_weakcipher_hardcoded_key_aes
def encrypt_hardcoded_key(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('AES-256-CBC')
  cipher.encrypt
  cipher.key = KEY # vuln-code-snippet vuln-line ruby_weakcipher_hardcoded_key_aes
  cipher.iv = cipher.random_iv
  ciphertext = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakcipher_hardcoded_key_aes
