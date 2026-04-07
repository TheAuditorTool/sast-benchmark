require_relative 'shared'

# vuln-code-snippet start ruby_weakrand_time_ns_token
def generate_token(req)
  token = "#{Time.now.nsec}#{rand}" # vuln-code-snippet vuln-line ruby_weakrand_time_ns_token
  BenchmarkResponse.json({ token: token })
end
# vuln-code-snippet end ruby_weakrand_time_ns_token
