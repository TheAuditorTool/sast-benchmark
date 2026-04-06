require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_weakrand_securerandom_uuid
def generate_uuid(req)
  id = SecureRandom.uuid # vuln-code-snippet safe-line ruby_weakrand_securerandom_uuid
  BenchmarkResponse.ok(id)
end
# vuln-code-snippet end ruby_weakrand_securerandom_uuid
