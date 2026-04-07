require_relative 'shared'
require 'openssl'

FIXED_KEY_32 = 'aaaabbbbccccddddeeeeffffgggghhhh'
FIXED_IV_16  = 'aaaabbbbccccdddd'

# vuln-code-snippet start ruby_weakcipher_reused_key_iv
def encrypt_reused_key_iv(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('AES-256-CBC')
  cipher.encrypt
  cipher.key = FIXED_KEY_32 # vuln-code-snippet vuln-line ruby_weakcipher_reused_key_iv
  cipher.iv  = FIXED_IV_16
  ciphertext = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakcipher_reused_key_iv
