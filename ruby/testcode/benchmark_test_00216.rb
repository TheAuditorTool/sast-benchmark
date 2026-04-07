require_relative 'shared'
require 'openssl'

module KMS
  def self.decrypt_data_key(encrypted_key)
    OpenSSL::Cipher.new('AES-256-GCM').random_key
  end
end

def encrypt_kms(req)
  plaintext = req.body_str
  data_key = KMS.decrypt_data_key(ENV.fetch('ENCRYPTED_DATA_KEY'))
  cipher = OpenSSL::Cipher.new('AES-256-GCM')
  cipher.encrypt
  cipher.key = data_key
  nonce = cipher.random_iv
  cipher.auth_data = ''
  ciphertext = cipher.update(plaintext) + cipher.final
  tag = cipher.auth_tag
  BenchmarkResponse.ok({ nonce: nonce.unpack1('H*'), tag: tag.unpack1('H*'), data: ciphertext.unpack1('H*') }.to_s)
end
