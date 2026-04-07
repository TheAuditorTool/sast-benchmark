require_relative 'shared'
require 'openssl'
require 'securerandom'

module SecretsManager
  def self.fetch(name)
    ENV.fetch(name.upcase)
  end
end

def encrypt_rotated_key(req)
  plaintext = req.body_str
  key = [SecretsManager.fetch('current_key')].pack('H*')
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
