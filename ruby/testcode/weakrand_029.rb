require_relative 'shared'

# vuln-code-snippet start ruby_weakrand_rand_bytes_manual
def generate_key(req)
  key = Array.new(16) { rand(256) }.pack('C*') # vuln-code-snippet vuln-line ruby_weakrand_rand_bytes_manual
  BenchmarkResponse.json({ key: key.unpack1('H*') })
end
# vuln-code-snippet end ruby_weakrand_rand_bytes_manual
