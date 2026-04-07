require_relative 'shared'

# vuln-code-snippet start ruby_weakrand_srand_pid
def generate_token(req)
  srand(Process.pid) # vuln-code-snippet vuln-line ruby_weakrand_srand_pid
  token = rand(2**128)
  BenchmarkResponse.json({ token: token })
end
# vuln-code-snippet end ruby_weakrand_srand_pid
