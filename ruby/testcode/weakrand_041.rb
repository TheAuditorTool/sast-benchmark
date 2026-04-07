require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_weakrand_securerandom_base64_32
def generate_token(req)
  token = SecureRandom.base64(32) # vuln-code-snippet safe-line ruby_weakrand_securerandom_base64_32
  BenchmarkResponse.json({ token: token })
end
# vuln-code-snippet end ruby_weakrand_securerandom_base64_32
