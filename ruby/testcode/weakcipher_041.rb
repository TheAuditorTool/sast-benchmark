require_relative 'shared'
require 'openssl'
require 'securerandom'

module AwsKmsClient
  def self.generate_data_key(key_id:)
    # Stub: in production this calls the AWS KMS GenerateDataKey API.
    # Returns a plaintext 32-byte key; ciphertext blob is stored separately.
    OpenSSL::Random.random_bytes(32)
  end
end

# vuln-code-snippet start ruby_weakcipher_kms_wrapped_key
def encrypt_kms_wrapped(req)
  plaintext = req.body_str
  key_data  = AwsKmsClient.generate_data_key(key_id: ENV.fetch('KMS_KEY_ID')) # vuln-code-snippet safe-line ruby_weakcipher_kms_wrapped_key
  cipher = OpenSSL::Cipher.new('AES-256-GCM')
  cipher.encrypt
  cipher.key = key_data
  cipher.iv  = SecureRandom.random_bytes(12)
  cipher.auth_data = ''
  ciphertext = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.json({ data: ciphertext.unpack1('H*') })
end
# vuln-code-snippet end ruby_weakcipher_kms_wrapped_key
