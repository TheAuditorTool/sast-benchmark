require_relative 'shared'

def generate_key_byte(req)
  key_byte = (Math.sin(42) * 256).abs.to_i
  BenchmarkResponse.json({ key_byte: key_byte })
end
