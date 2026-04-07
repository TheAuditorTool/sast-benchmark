require_relative 'shared'
require 'openssl'
require 'securerandom'

module SecretsManager
  def self.fetch(name)
    # Stub: in production, fetches from AWS Secrets Manager / HashiCorp Vault.
    # Key is rotated on a schedule; no key material is present in source code.
    ENV.fetch(name.upcase)
  end
end

# vuln-code-snippet start ruby_weakcipher_secure_random_key_rotation
def encrypt_rotated_key(req)
  plaintext = req.body_str
  # Key is rotated from secrets manager; no hardcoded key material present.
  key = [SecretsManager.fetch('current_key')].pack('H*') # vuln-code-snippet safe-line ruby_weakcipher_secure_random_key_rotation
  iv  = SecureRandom.random_bytes(12)
  cipher = OpenSSL::Cipher.new('AES-256-GCM')
  cipher.encrypt
  cipher.key      = key
  cipher.iv       = iv
  cipher.auth_data = ''
  ciphertext = cipher.update(plaintext) + cipher.final
  tag        = cipher.auth_tag
  BenchmarkResponse.json({ iv: iv.unpack1('H*'), data: ciphertext.unpack1('H*'), tag: tag.unpack1('H*') })
end
# vuln-code-snippet end ruby_weakcipher_secure_random_key_rotation
