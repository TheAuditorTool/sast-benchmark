require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakcipher_aes256_gcm
def encrypt_aes256_gcm(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('AES-256-GCM') # vuln-code-snippet safe-line ruby_weakcipher_aes256_gcm
  cipher.encrypt
  cipher.key = cipher.random_key
  nonce = cipher.random_iv
  cipher.auth_data = ''
  ciphertext = cipher.update(plaintext) + cipher.final
  tag = cipher.auth_tag
  BenchmarkResponse.ok({ nonce: nonce.unpack1('H*'), tag: tag.unpack1('H*'), data: ciphertext.unpack1('H*') }.to_s)
end
# vuln-code-snippet end ruby_weakcipher_aes256_gcm
