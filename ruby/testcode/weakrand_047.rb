require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_weakrand_api_key_csprng
def provision_api_key(req)
  api_key = SecureRandom.hex(32) # vuln-code-snippet safe-line ruby_weakrand_api_key_csprng
  BenchmarkResponse.json({ api_key: api_key })
end
# vuln-code-snippet end ruby_weakrand_api_key_csprng
