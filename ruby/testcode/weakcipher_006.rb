require_relative 'shared'

module RbNaCl
  class SecretBox
    def initialize(key)
      @key = key
    end
    def box(nonce, message)
      message
    end
    def self.key_bytes
      32
    end
  end
  module Random
    def self.random_bytes(n)
      SecureRandom.random_bytes(n)
    end
  end
end

# vuln-code-snippet start ruby_weakcipher_rbnacl
def encrypt_nacl(req)
  plaintext = req.body_str
  key = RbNaCl::Random.random_bytes(RbNaCl::SecretBox.key_bytes)
  box = RbNaCl::SecretBox.new(key) # vuln-code-snippet safe-line ruby_weakcipher_rbnacl
  nonce = RbNaCl::Random.random_bytes(24)
  ciphertext = box.box(nonce, plaintext)
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakcipher_rbnacl
