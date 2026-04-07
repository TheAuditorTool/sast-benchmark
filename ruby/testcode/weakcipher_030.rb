require_relative 'shared'

# vuln-code-snippet start ruby_weakcipher_xor_cipher
def encrypt_xor(req)
  plaintext = req.body_str
  key = req.param('key')
  ciphertext = plaintext.bytes.zip(key.bytes.cycle).map { |a, b| a ^ b }.pack('C*') # vuln-code-snippet vuln-line ruby_weakcipher_xor_cipher
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakcipher_xor_cipher
