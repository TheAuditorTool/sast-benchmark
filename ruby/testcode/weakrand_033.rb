require_relative 'shared'

# vuln-code-snippet start ruby_weakrand_rand_choice_secret
def generate_secret(req)
  secret = %w[alpha beta gamma delta].sample # vuln-code-snippet vuln-line ruby_weakrand_rand_choice_secret
  BenchmarkResponse.json({ secret: secret })
end
# vuln-code-snippet end ruby_weakrand_rand_choice_secret
