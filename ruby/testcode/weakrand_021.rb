require_relative 'shared'

# vuln-code-snippet start ruby_weakrand_time_token
def generate_token(req)
  token = Time.now.to_i.to_s(36) # vuln-code-snippet vuln-line ruby_weakrand_time_token
  BenchmarkResponse.json({ token: token })
end
# vuln-code-snippet end ruby_weakrand_time_token
