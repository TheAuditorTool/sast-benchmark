require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakcipher_null_key_derived
def encrypt_weak_kdf(req)
  plaintext = req.body_str
  password = req.param('password')
  key = OpenSSL::PKCS5.pbkdf2_hmac_sha1(password, '', 1, 32) # vuln-code-snippet vuln-line ruby_weakcipher_null_key_derived
  cipher = OpenSSL::Cipher.new('AES-256-CBC')
  cipher.encrypt
  cipher.key = key
  cipher.iv = cipher.random_iv
  ciphertext = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakcipher_null_key_derived
