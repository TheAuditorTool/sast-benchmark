require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_weakrand_securerandom_uuid_v4
def generate_token(req)
  token = SecureRandom.uuid # vuln-code-snippet safe-line ruby_weakrand_securerandom_uuid_v4
  BenchmarkResponse.json({ token: token })
end
# vuln-code-snippet end ruby_weakrand_securerandom_uuid_v4
