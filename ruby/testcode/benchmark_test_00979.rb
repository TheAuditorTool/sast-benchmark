require_relative 'shared'

begin
  require 'lockbox'
rescue LoadError
end

def encrypt_lockbox(req)
  data = req.body_str
  box  = Lockbox.new(key: ENV.fetch('LOCKBOX_KEY'))
  ciphertext = box.encrypt(data)
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
