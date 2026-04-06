require_relative 'shared'

# vuln-code-snippet start ruby_weakrand_array_sample
def generate_token_sample(req)
  chars = ('a'..'z').to_a + ('0'..'9').to_a
  token = chars.sample(32).join # vuln-code-snippet vuln-line ruby_weakrand_array_sample
  BenchmarkResponse.ok(token)
end
# vuln-code-snippet end ruby_weakrand_array_sample
