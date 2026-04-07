require_relative 'shared'
require 'openssl'
require 'securerandom'

# vuln-code-snippet start ruby_weakcipher_openssl_kdf_hkdf
def derive_key_hkdf(req)
  ikm  = OpenSSL::Random.random_bytes(32)
  salt = SecureRandom.random_bytes(16)
  key  = OpenSSL::KDF.hkdf(ikm, salt: salt, info: 'encryption', length: 32, hash: 'SHA256') # vuln-code-snippet safe-line ruby_weakcipher_openssl_kdf_hkdf
  cipher = OpenSSL::Cipher.new('AES-256-GCM')
  cipher.encrypt
  cipher.key = key
  cipher.iv  = SecureRandom.random_bytes(12)
  cipher.auth_data = ''
  ciphertext = cipher.update(req.body_str) + cipher.final
  BenchmarkResponse.json({ data: ciphertext.unpack1('H*') })
end
# vuln-code-snippet end ruby_weakcipher_openssl_kdf_hkdf
