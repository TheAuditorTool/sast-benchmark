require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakcipher_aes128_ecb
def encrypt_aes128_ecb(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('AES-128-ECB') # vuln-code-snippet vuln-line ruby_weakcipher_aes128_ecb
  cipher.encrypt
  cipher.key = cipher.random_key
  ciphertext = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakcipher_aes128_ecb
