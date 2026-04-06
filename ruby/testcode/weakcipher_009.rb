require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakcipher_3des
def encrypt_3des(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('DES-EDE3-CBC') # vuln-code-snippet vuln-line ruby_weakcipher_3des
  cipher.encrypt
  cipher.key = cipher.random_key
  cipher.iv = cipher.random_iv
  encrypted = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(encrypted.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakcipher_3des
