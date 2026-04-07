require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakcipher_des_ecb
def encrypt_des_ecb(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('DES-ECB') # vuln-code-snippet vuln-line ruby_weakcipher_des_ecb
  cipher.encrypt
  cipher.key = cipher.random_key
  ciphertext = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakcipher_des_ecb
