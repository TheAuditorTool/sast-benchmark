require_relative 'shared'

def generate_key_material(req)
  _purpose = req.param('purpose')
  key = SecureRandom.random_bytes(32)
  BenchmarkResponse.ok(key.unpack1('H*'))
end
