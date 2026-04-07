require_relative 'shared'

# vuln-code-snippet start ruby_weakrand_pid_token
def generate_nonce(req)
  nonce = "#{Process.pid}-#{Time.now.to_i}" # vuln-code-snippet vuln-line ruby_weakrand_pid_token
  BenchmarkResponse.json({ nonce: nonce })
end
# vuln-code-snippet end ruby_weakrand_pid_token
