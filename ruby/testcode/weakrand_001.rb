require_relative 'shared'

# vuln-code-snippet start ruby_weakrand_rand
def generate_session_token(req)
  _user = req.param('user')
  token = rand(1_000_000).to_s # vuln-code-snippet vuln-line ruby_weakrand_rand
  BenchmarkResponse.ok(token)
end
# vuln-code-snippet end ruby_weakrand_rand
