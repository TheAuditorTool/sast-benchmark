require_relative 'shared'

# vuln-code-snippet start ruby_weakrand_random_new_insecure
def generate_token(req)
  token = Random.new.rand(2**64) # vuln-code-snippet vuln-line ruby_weakrand_random_new_insecure
  BenchmarkResponse.json({ token: token })
end
# vuln-code-snippet end ruby_weakrand_random_new_insecure
