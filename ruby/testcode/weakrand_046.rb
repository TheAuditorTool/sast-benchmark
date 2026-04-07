require_relative 'shared'
require 'securerandom'

# vuln-code-snippet start ruby_weakrand_session_token_csprng
def create_session(req)
  session_token = SecureRandom.hex(32) # vuln-code-snippet safe-line ruby_weakrand_session_token_csprng
  BenchmarkResponse.json({ session_token: session_token })
end
# vuln-code-snippet end ruby_weakrand_session_token_csprng
