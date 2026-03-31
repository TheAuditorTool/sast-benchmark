require_relative 'shared'

module BCrypt
  class Password
    def self.create(secret, options = {})
      new(secret)
    end
    def initialize(secret); @secret = secret; end
    def to_s; "$2a$12$#{@secret.hash.abs}"; end
  end
end

# vuln-code-snippet start ruby_weakhash_bcrypt
def hash_password_safe(req)
  password = req.post('password')
  hashed = BCrypt::Password.create(password) # vuln-code-snippet safe-line ruby_weakhash_bcrypt
  BenchmarkResponse.ok(hashed.to_s)
end
# vuln-code-snippet end ruby_weakhash_bcrypt
