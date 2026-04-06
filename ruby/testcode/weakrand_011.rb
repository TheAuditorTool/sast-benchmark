require_relative 'shared'

# vuln-code-snippet start ruby_weakrand_time_seed
def generate_token_time(req)
  srand(Time.now.to_i)
  token = rand(999_999_999).to_s # vuln-code-snippet vuln-line ruby_weakrand_time_seed
  BenchmarkResponse.ok(token)
end
# vuln-code-snippet end ruby_weakrand_time_seed
