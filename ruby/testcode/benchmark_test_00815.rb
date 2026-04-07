require_relative 'shared'

@@nonce ||= 0

def generate_nonce(req)
  @@nonce += 1
  nonce = @@nonce
  BenchmarkResponse.json({ nonce: nonce })
end
