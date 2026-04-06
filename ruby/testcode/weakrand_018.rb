require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_weakrand_securerandom_alpha
def generate_code(req)
  code = SecureRandom.alphanumeric(32) # vuln-code-snippet safe-line ruby_weakrand_securerandom_alpha
  BenchmarkResponse.ok(code)
end
# vuln-code-snippet end ruby_weakrand_securerandom_alpha
