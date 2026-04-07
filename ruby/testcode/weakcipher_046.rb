require_relative 'shared'
require 'openssl'
require 'securerandom'

# vuln-code-snippet start ruby_weakcipher_aes256_gcm_env_key
def encrypt_env_key_gcm(req)
  plaintext = req.body_str
  key    = [ENV.fetch('ENCRYPTION_KEY')].pack('H*')
  iv     = SecureRandom.random_bytes(12) # vuln-code-snippet safe-line ruby_weakcipher_aes256_gcm_env_key
  cipher = OpenSSL::Cipher.new('AES-256-GCM')
  cipher.encrypt
  cipher.key      = key
  cipher.iv       = iv
  cipher.auth_data = ''
  ciphertext = cipher.update(plaintext) + cipher.final
  tag        = cipher.auth_tag
  BenchmarkResponse.json({ iv: iv.unpack1('H*'), data: ciphertext.unpack1('H*'), tag: tag.unpack1('H*') })
end
# vuln-code-snippet end ruby_weakcipher_aes256_gcm_env_key
