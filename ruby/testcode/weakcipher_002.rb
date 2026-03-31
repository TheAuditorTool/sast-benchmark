require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakcipher_aes_gcm
def encrypt_data_safe(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('AES-256-GCM') # vuln-code-snippet safe-line ruby_weakcipher_aes_gcm
  cipher.encrypt
  cipher.key = cipher.random_key
  iv = cipher.random_iv
  encrypted = cipher.update(plaintext) + cipher.final
  tag = cipher.auth_tag
  BenchmarkResponse.ok({ iv: iv.unpack1('H*'), data: encrypted.unpack1('H*'), tag: tag.unpack1('H*') }.to_s)
end
# vuln-code-snippet end ruby_weakcipher_aes_gcm
