require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakcipher_blowfish_hardcoded_key
def encrypt_blowfish(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('BF-CBC')
  cipher.encrypt
  cipher.key = 'hardcoded_blowfish_key_16bytes!!' # vuln-code-snippet vuln-line ruby_weakcipher_blowfish_hardcoded_key
  cipher.iv = cipher.random_iv
  ciphertext = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakcipher_blowfish_hardcoded_key
