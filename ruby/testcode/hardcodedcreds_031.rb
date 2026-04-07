require 'openssl'
require_relative 'shared'

ENCRYPTION_KEY = "0123456789abcdef0123456789abcdef"

# vuln-code-snippet start ruby_hardcoded_encryption_key
def encrypt_data_handler(req)
  cipher = OpenSSL::Cipher.new('AES-256-CBC')
  cipher.encrypt
  cipher.key = ENCRYPTION_KEY  # vuln-code-snippet vuln-line ruby_hardcoded_encryption_key
  cipher.iv = cipher.random_iv
  encrypted = cipher.update(req[:data].to_s) + cipher.final
  BenchmarkResponse.json({ ciphertext: encrypted.unpack1('H*') })
end
# vuln-code-snippet end ruby_hardcoded_encryption_key
