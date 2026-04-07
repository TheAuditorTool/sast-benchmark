require_relative 'shared'
require 'digest'

# vuln-code-snippet start ruby_weakrand_hash_rand
def generate_token(req)
  token = Digest::SHA256.hexdigest(rand.to_s) # vuln-code-snippet vuln-line ruby_weakrand_hash_rand
  BenchmarkResponse.json({ token: token })
end
# vuln-code-snippet end ruby_weakrand_hash_rand
