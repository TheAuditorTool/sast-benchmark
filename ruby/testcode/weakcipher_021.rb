require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakcipher_aes_ecb_256
def encrypt_ecb_256(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('AES-256-ECB') # vuln-code-snippet vuln-line ruby_weakcipher_aes_ecb_256
  cipher.encrypt
  cipher.key = cipher.random_key
  ciphertext = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakcipher_aes_ecb_256
