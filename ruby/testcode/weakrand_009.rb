require_relative 'shared'

# vuln-code-snippet start ruby_weakrand_shuffle_token
def generate_token_shuffle(req)
  charset = ('A'..'Z').to_a + ('a'..'z').to_a + ('0'..'9').to_a
  token = charset.shuffle.first(32).join # vuln-code-snippet vuln-line ruby_weakrand_shuffle_token
  BenchmarkResponse.ok(token)
end
# vuln-code-snippet end ruby_weakrand_shuffle_token
