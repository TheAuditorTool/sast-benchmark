require_relative 'shared'

def generate_nonce(req)
  nonce = "#{Process.pid}-#{Time.now.to_i}"
  BenchmarkResponse.json({ nonce: nonce })
end
