require_relative 'shared'
require 'openssl'

FIXED_KEY_32 = 'aaaabbbbccccddddeeeeffffgggghhhh'
FIXED_IV_16  = 'aaaabbbbccccdddd'

def encrypt_reused_key_iv(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('AES-256-CBC')
  cipher.encrypt
  cipher.key = FIXED_KEY_32
  cipher.iv  = FIXED_IV_16
  ciphertext = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
