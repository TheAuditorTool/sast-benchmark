require_relative 'shared'

begin
  require 'lockbox'
rescue LoadError
  # lockbox gem not available -- syntax-checked only
end

# vuln-code-snippet start ruby_weakcipher_lockbox_encrypt
def encrypt_lockbox(req)
  data = req.body_str
  box  = Lockbox.new(key: ENV.fetch('LOCKBOX_KEY')) # vuln-code-snippet safe-line ruby_weakcipher_lockbox_encrypt
  ciphertext = box.encrypt(data)
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakcipher_lockbox_encrypt
