require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakcipher_session_iv
def encrypt_session_iv(req)
  plaintext = req.body_str
  key = OpenSSL::Random.random_bytes(32)
  cipher = OpenSSL::Cipher.new('AES-256-CBC')
  cipher.encrypt
  cipher.key = key
  iv = req.cookie('session_id')[0..15] # vuln-code-snippet vuln-line ruby_weakcipher_session_iv
  cipher.iv = iv.ljust(16, "\x00")
  ciphertext = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakcipher_session_iv
