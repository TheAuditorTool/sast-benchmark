require_relative 'shared'

# vuln-code-snippet start ruby_weakrand_random_rand
def generate_csrf_token(req)
  _session = req.cookie('session')
  token = Random.rand(999999) # vuln-code-snippet vuln-line ruby_weakrand_random_rand
  BenchmarkResponse.ok(token.to_s)
end
# vuln-code-snippet end ruby_weakrand_random_rand
