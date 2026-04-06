require_relative 'shared'
require 'securerandom'

module RbNaCl
  class SecretBox
    def initialize(key); @key = key; end
    def box(nonce, msg); msg; end
    def self.key_bytes; 32; end
    def self.nonce_bytes; 24; end
  end
  module Random
    def self.random_bytes(n); SecureRandom.random_bytes(n); end
  end
end

# vuln-code-snippet start ruby_weakcipher_nacl_box
def encrypt_nacl_secret(req)
  plaintext = req.body_str
  key = RbNaCl::Random.random_bytes(RbNaCl::SecretBox.key_bytes)
  box = RbNaCl::SecretBox.new(key) # vuln-code-snippet safe-line ruby_weakcipher_nacl_box
  nonce = RbNaCl::Random.random_bytes(RbNaCl::SecretBox.nonce_bytes)
  ciphertext = box.box(nonce, plaintext)
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakcipher_nacl_box
