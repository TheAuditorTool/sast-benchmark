require_relative 'shared'

# vuln-code-snippet start ruby_weakhash_xor_bytes
def store_password_xor(req)
  password = req.param('password')
  key      = req.param('key')
  digest = password.bytes.zip(key.bytes.cycle).map { |a, b| a ^ b }.pack('C*') # vuln-code-snippet vuln-line ruby_weakhash_xor_bytes
  BenchmarkResponse.json({ hash: digest.unpack1('H*') })
end
# vuln-code-snippet end ruby_weakhash_xor_bytes
