require_relative 'shared'

# vuln-code-snippet start ruby_weakrand_float_key
def generate_key(req)
  key = (rand * 1_000_000_000).to_i.to_s(16) # vuln-code-snippet vuln-line ruby_weakrand_float_key
  BenchmarkResponse.json({ key: key })
end
# vuln-code-snippet end ruby_weakrand_float_key
