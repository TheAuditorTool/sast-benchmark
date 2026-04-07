require_relative 'shared'

# vuln-code-snippet start ruby_weakrand_sin_float
def generate_key_byte(req)
  key_byte = (Math.sin(42) * 256).abs.to_i # vuln-code-snippet vuln-line ruby_weakrand_sin_float
  BenchmarkResponse.json({ key_byte: key_byte })
end
# vuln-code-snippet end ruby_weakrand_sin_float
