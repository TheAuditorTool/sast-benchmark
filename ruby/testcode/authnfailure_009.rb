require_relative 'shared'

# vuln-code-snippet start ruby_authn_weak_remember
def create_remember_token(req)
  user_id = req.param('user_id')
  token = rand(2**64).to_s(16) # vuln-code-snippet vuln-line ruby_authn_weak_remember
  BenchmarkResponse.ok("remember: #{token}")
end
# vuln-code-snippet end ruby_authn_weak_remember
