require_relative 'shared'
require 'openssl'
require 'securerandom'

# vuln-code-snippet start ruby_weakcipher_openssl_aead_tag
def encrypt_gcm_with_tag(req)
  plaintext = req.body_str
  key    = OpenSSL::Random.random_bytes(32)
  iv     = SecureRandom.random_bytes(12)
  cipher = OpenSSL::Cipher.new('AES-256-GCM')
  cipher.encrypt
  cipher.key      = key
  cipher.iv       = iv
  cipher.auth_data = ''
  ciphertext = cipher.update(plaintext) + cipher.final
  auth_tag   = cipher.auth_tag # vuln-code-snippet safe-line ruby_weakcipher_openssl_aead_tag
  BenchmarkResponse.json({ data: ciphertext.unpack1('H*'), tag: auth_tag.unpack1('H*'), iv: iv.unpack1('H*') })
end

def decrypt_gcm_with_tag(ciphertext_hex, tag_hex, iv_hex, key)
  cipher = OpenSSL::Cipher.new('AES-256-GCM')
  cipher.decrypt
  cipher.key      = key
  cipher.iv       = [iv_hex].pack('H*')
  cipher.auth_tag = [tag_hex].pack('H*')
  cipher.auth_data = ''
  cipher.update([ciphertext_hex].pack('H*')) + cipher.final
end
# vuln-code-snippet end ruby_weakcipher_openssl_aead_tag
