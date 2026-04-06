require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakcipher_blowfish
def encrypt_blowfish(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('BF-CBC') # vuln-code-snippet vuln-line ruby_weakcipher_blowfish
  cipher.encrypt
  cipher.key = cipher.random_key
  encrypted = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(encrypted.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakcipher_blowfish
