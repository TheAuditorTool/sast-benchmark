require_relative 'shared'
require 'openssl'
require 'digest'

# vuln-code-snippet start ruby_weakcipher_md5_key_derivation
def encrypt_md5_key(req)
  password = req.param('password')
  plaintext = req.body_str
  key = Digest::MD5.digest(password) # vuln-code-snippet vuln-line ruby_weakcipher_md5_key_derivation
  cipher = OpenSSL::Cipher.new('AES-128-CBC')
  cipher.encrypt
  cipher.key = key
  cipher.iv = cipher.random_iv
  ciphertext = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakcipher_md5_key_derivation
