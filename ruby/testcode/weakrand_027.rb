require_relative 'shared'

# vuln-code-snippet start ruby_weakrand_mod_user_count
def generate_nonce(req)
  count = 42
  nonce = count % 1000 # vuln-code-snippet vuln-line ruby_weakrand_mod_user_count
  BenchmarkResponse.json({ nonce: nonce })
end
# vuln-code-snippet end ruby_weakrand_mod_user_count
