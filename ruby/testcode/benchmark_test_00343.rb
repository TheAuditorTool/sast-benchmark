require_relative 'shared'

def generate_key(req)
  key = Array.new(16) { rand(256) }.pack('C*')
  BenchmarkResponse.json({ key: key.unpack1('H*') })
end
