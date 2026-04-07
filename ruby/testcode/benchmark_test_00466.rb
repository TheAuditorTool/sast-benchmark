require_relative 'shared'

def encrypt_xor(req)
  plaintext = req.body_str
  key = req.param('key')
  ciphertext = plaintext.bytes.zip(key.bytes.cycle).map { |a, b| a ^ b }.pack('C*')
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
