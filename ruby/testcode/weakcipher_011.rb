require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakcipher_rc2
def encrypt_rc2(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('RC2-CBC') # vuln-code-snippet vuln-line ruby_weakcipher_rc2
  cipher.encrypt
  cipher.key = cipher.random_key
  cipher.iv = cipher.random_iv
  encrypted = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(encrypted.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakcipher_rc2
