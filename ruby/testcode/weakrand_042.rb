require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_weakrand_securerandom_number_big
def generate_token(req)
  token = SecureRandom.random_number(2**128) # vuln-code-snippet safe-line ruby_weakrand_securerandom_number_big
  BenchmarkResponse.json({ token: token })
end
# vuln-code-snippet end ruby_weakrand_securerandom_number_big
