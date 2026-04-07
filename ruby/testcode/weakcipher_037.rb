require_relative 'shared'
require 'openssl'
require 'securerandom'

# OpenSSL >= 1.1.0
# vuln-code-snippet start ruby_weakcipher_chacha20_poly1305
def encrypt_chacha20(req)
  plaintext = req.body_str
  key = OpenSSL::Random.random_bytes(32)
  nonce = SecureRandom.random_bytes(12) # vuln-code-snippet safe-line ruby_weakcipher_chacha20_poly1305
  cipher = OpenSSL::Cipher.new('chacha20-poly1305')
  cipher.encrypt
  cipher.key = key
  cipher.iv = nonce
  cipher.auth_data = ''
  ciphertext = cipher.update(plaintext) + cipher.final
  tag = cipher.auth_tag
  BenchmarkResponse.json({ data: ciphertext.unpack1('H*'), tag: tag.unpack1('H*') })
end
# vuln-code-snippet end ruby_weakcipher_chacha20_poly1305
