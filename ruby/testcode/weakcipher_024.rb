require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakcipher_srand_key_gen
def encrypt_seeded_key(req)
  plaintext = req.body_str
  srand(42)
  key = Array.new(32) { rand(256) }.pack('C*') # vuln-code-snippet vuln-line ruby_weakcipher_srand_key_gen
  cipher = OpenSSL::Cipher.new('AES-256-CBC')
  cipher.encrypt
  cipher.key = key
  cipher.iv = cipher.random_iv
  ciphertext = cipher.update(plaintext) + cipher.final
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakcipher_srand_key_gen
