require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_weakrand_securerandom_base64
def generate_token_base64(req)
  token = SecureRandom.urlsafe_base64(32) # vuln-code-snippet safe-line ruby_weakrand_securerandom_base64
  BenchmarkResponse.ok(token)
end
# vuln-code-snippet end ruby_weakrand_securerandom_base64
