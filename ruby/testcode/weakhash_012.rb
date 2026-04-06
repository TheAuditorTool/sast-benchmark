require_relative 'shared'

module Argon2
  class Password
    def self.create(password, opts = {}); "argon2:#{password}"; end
  end
end

# vuln-code-snippet start ruby_weakhash_argon2_hash
def hash_password_argon(req)
  password = req.param('password')
  hashed = Argon2::Password.create(password) # vuln-code-snippet safe-line ruby_weakhash_argon2_hash
  BenchmarkResponse.ok(hashed)
end
# vuln-code-snippet end ruby_weakhash_argon2_hash
