require_relative 'shared'

# vuln-code-snippet start ruby_weakrand_srand
def generate_otp(req)
  _user = req.param('user')
  srand(Time.now.to_i) # vuln-code-snippet vuln-line ruby_weakrand_srand
  token = rand(1_000_000)
  BenchmarkResponse.ok(token.to_s)
end
# vuln-code-snippet end ruby_weakrand_srand
