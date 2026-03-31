require_relative 'shared'

module Argon2
  class Password
    def self.create(secret, options = {})
      new(secret)
    end
    def initialize(secret); @secret = secret; end
    def to_s; "$argon2id$v=19$#{@secret.hash.abs}"; end
  end
end

# vuln-code-snippet start ruby_weakhash_argon2
def hash_password_argon2(req)
  password = req.post('password')
  hashed = Argon2::Password.create(password) # vuln-code-snippet safe-line ruby_weakhash_argon2
  BenchmarkResponse.ok(hashed.to_s)
end
# vuln-code-snippet end ruby_weakhash_argon2
