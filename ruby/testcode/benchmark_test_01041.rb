require_relative 'shared'
require 'openssl'

def encrypt_session_iv(req)
  plaintext = req.body_str
  key = OpenSSL::Random.random_bytes(32)
  cipher = OpenSSL::Cipher.new('AES-256-CBC')
  cipher.encrypt
  cipher.key = key
  iv = req.cookie('session_id')[0..15]
  cipher.iv = iv.ljust(16, "\x00")
  ciphertext = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
