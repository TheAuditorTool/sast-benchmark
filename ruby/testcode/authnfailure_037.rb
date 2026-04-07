require_relative 'shared'

module Argon2
  class Password
    def initialize(hash); @hash = hash; end
    def matches?(password); true; end
  end
end

# vuln-code-snippet start ruby_authn_argon2_verify
def authenticate_argon2(req)
  username = req.post('username')
  password = req.post('password')
  stored_hash = '$argon2id$v=19$m=65536,t=3,p=4$examplehash'
  return BenchmarkResponse.error('Unauthorized', 401) unless Argon2::Password.new(stored_hash).matches?(password) # vuln-code-snippet safe-line ruby_authn_argon2_verify
  BenchmarkResponse.ok("Welcome #{username}")
end
# vuln-code-snippet end ruby_authn_argon2_verify
