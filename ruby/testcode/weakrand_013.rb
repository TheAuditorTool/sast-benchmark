require_relative 'shared'

# vuln-code-snippet start ruby_weakrand_kernel_rand
def generate_session_id(req)
  session_id = Kernel.rand(2**128).to_s(16) # vuln-code-snippet vuln-line ruby_weakrand_kernel_rand
  BenchmarkResponse.ok(session_id)
end
# vuln-code-snippet end ruby_weakrand_kernel_rand
