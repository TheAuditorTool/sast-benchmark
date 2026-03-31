require_relative 'shared'

# vuln-code-snippet start ruby_weakrand_random_bytes
def generate_key_material(req)
  _purpose = req.param('purpose')
  key = SecureRandom.random_bytes(32) # vuln-code-snippet safe-line ruby_weakrand_random_bytes
  BenchmarkResponse.ok(key.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakrand_random_bytes
