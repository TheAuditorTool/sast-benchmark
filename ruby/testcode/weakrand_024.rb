require_relative 'shared'

# vuln-code-snippet start ruby_weakrand_sequential_nonce
@@nonce ||= 0

def generate_nonce(req)
  @@nonce += 1
  nonce = @@nonce # vuln-code-snippet vuln-line ruby_weakrand_sequential_nonce
  BenchmarkResponse.json({ nonce: nonce })
end
# vuln-code-snippet end ruby_weakrand_sequential_nonce
