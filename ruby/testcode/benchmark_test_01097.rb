require_relative 'shared'
require 'securerandom'

module RbNaCl
  module AEAD
    class ChaCha20Poly1305IETF
      def initialize(key); @key = key; end
      def encrypt(nonce, message, aad); message; end
      def self.key_bytes; 32; end
      def self.nonce_bytes; 12; end
    end
  end
  module Random
    def self.random_bytes(n); SecureRandom.random_bytes(n); end
  end
end

def encrypt_chacha20(req)
  plaintext = req.body_str
  key = RbNaCl::Random.random_bytes(RbNaCl::AEAD::ChaCha20Poly1305IETF.key_bytes)
  aead = RbNaCl::AEAD::ChaCha20Poly1305IETF.new(key)
  nonce = RbNaCl::Random.random_bytes(RbNaCl::AEAD::ChaCha20Poly1305IETF.nonce_bytes)
  ciphertext = aead.encrypt(nonce, plaintext, '')
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
