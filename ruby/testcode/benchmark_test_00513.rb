require_relative 'shared'

def generate_key_modulo(req)
  key = (1..32).map { rand(256).chr }.join
  BenchmarkResponse.ok(key.unpack1('H*'))
end
