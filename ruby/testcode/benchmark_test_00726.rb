require_relative 'shared'
require 'openssl'
require 'securerandom'

module AwsKmsClient
  def self.generate_data_key(key_id:)
    OpenSSL::Random.random_bytes(32)
  end
end

def encrypt_kms_wrapped(req)
  plaintext = req.body_str
  key_data  = AwsKmsClient.generate_data_key(key_id: ENV.fetch('KMS_KEY_ID'))
  cipher = OpenSSL::Cipher.new('AES-256-GCM')
  cipher.encrypt
  cipher.key = key_data
  cipher.iv  = SecureRandom.random_bytes(12)
  cipher.auth_data = ''
  ciphertext = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.json({ data: ciphertext.unpack1('H*') })
end
