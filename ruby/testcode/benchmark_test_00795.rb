require_relative 'shared'

def generate_nonce(req)
  count = 42
  nonce = count % 1000
  BenchmarkResponse.json({ nonce: nonce })
end
