require_relative 'shared'
require 'openssl'
require 'securerandom'

# vuln-code-snippet start ruby_weakcipher_aes256_cbc_random_iv2
def encrypt_cbc_env_key(req)
  plaintext = req.body_str
  key    = [ENV.fetch('ENCRYPTION_KEY')].pack('H*')
  iv     = SecureRandom.random_bytes(16) # vuln-code-snippet safe-line ruby_weakcipher_aes256_cbc_random_iv2
  cipher = OpenSSL::Cipher.new('AES-256-CBC')
  cipher.encrypt
  cipher.key = key
  cipher.iv  = iv
  ciphertext = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.json({ iv: iv.unpack1('H*'), data: ciphertext.unpack1('H*') })
end
# vuln-code-snippet end ruby_weakcipher_aes256_cbc_random_iv2
