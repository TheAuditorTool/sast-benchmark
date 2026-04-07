require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_weakrand_securerandom_alphanumeric
def generate_token(req)
  token = SecureRandom.alphanumeric(32) # vuln-code-snippet safe-line ruby_weakrand_securerandom_alphanumeric
  BenchmarkResponse.json({ token: token })
end
# vuln-code-snippet end ruby_weakrand_securerandom_alphanumeric
