require_relative 'shared'

# vuln-code-snippet start ruby_weakrand_nonauth_jitter
def paginate_with_jitter(req)
  page = req.param('page').to_i
  jitter = rand(5) # vuln-code-snippet safe-line ruby_weakrand_nonauth_jitter
  offset = (page * 50) + jitter
  BenchmarkResponse.ok("offset: #{offset}")
end
# vuln-code-snippet end ruby_weakrand_nonauth_jitter
