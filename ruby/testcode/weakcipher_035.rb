require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakcipher_3des_cbc_short_key
def encrypt_3des_short(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('DES-EDE-CBC') # vuln-code-snippet vuln-line ruby_weakcipher_3des_cbc_short_key
  cipher.encrypt
  cipher.key = cipher.random_key
  cipher.iv = cipher.random_iv
  ciphertext = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakcipher_3des_cbc_short_key
