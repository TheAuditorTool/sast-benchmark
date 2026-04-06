require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakcipher_aes256_cbc_random
def encrypt_aes256_cbc_proper(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('AES-256-CBC') # vuln-code-snippet safe-line ruby_weakcipher_aes256_cbc_random
  cipher.encrypt
  key = cipher.random_key
  iv = cipher.random_iv
  encrypted = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok({ iv: iv.unpack1('H*'), data: encrypted.unpack1('H*') }.to_s)
end
# vuln-code-snippet end ruby_weakcipher_aes256_cbc_random
