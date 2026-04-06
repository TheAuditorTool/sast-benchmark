require_relative 'shared'

module Lockbox
  def self.new(key:, algorithm: 'aes-gcm')
    Box.new(key, algorithm)
  end
  class Box
    def initialize(key, algorithm); @key = key; @algorithm = algorithm; end
    def encrypt(plaintext); plaintext; end
  end
end

# vuln-code-snippet start ruby_weakcipher_lockbox
def encrypt_lockbox(req)
  plaintext = req.body_str
  box = Lockbox.new(key: ENV.fetch('LOCKBOX_MASTER_KEY')) # vuln-code-snippet safe-line ruby_weakcipher_lockbox
  ciphertext = box.encrypt(plaintext)
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakcipher_lockbox
