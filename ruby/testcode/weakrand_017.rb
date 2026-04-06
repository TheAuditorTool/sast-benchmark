require_relative 'shared'

# vuln-code-snippet start ruby_weakrand_modulo_key
def generate_key_modulo(req)
  key = (1..32).map { rand(256).chr }.join # vuln-code-snippet vuln-line ruby_weakrand_modulo_key
  BenchmarkResponse.ok(key.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakrand_modulo_key
