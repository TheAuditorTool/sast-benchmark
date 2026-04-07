require_relative 'shared'

def generate_key(req)
  key = (rand * 1_000_000_000).to_i.to_s(16)
  BenchmarkResponse.json({ key: key })
end
