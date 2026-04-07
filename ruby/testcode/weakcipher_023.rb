require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakcipher_static_nonce_gcm
def encrypt_static_nonce(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('AES-256-GCM')
  cipher.encrypt
  cipher.key = cipher.random_key
  cipher.iv = "\x00" * 12 # vuln-code-snippet vuln-line ruby_weakcipher_static_nonce_gcm
  cipher.auth_data = ''
  ciphertext = cipher.update(plaintext) + cipher.final
  tag = cipher.auth_tag
  BenchmarkResponse.ok({ data: ciphertext.unpack1('H*'), tag: tag.unpack1('H*') }.to_s)
end
# vuln-code-snippet end ruby_weakcipher_static_nonce_gcm
