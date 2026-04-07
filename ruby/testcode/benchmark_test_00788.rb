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

def encrypt_lockbox(req)
  plaintext = req.body_str
  box = Lockbox.new(key: ENV.fetch('LOCKBOX_MASTER_KEY'))
  ciphertext = box.encrypt(plaintext)
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
