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

def hash_password_safe(req)
  password = req.post('password')
  hashed = BCrypt::Password.create(password)
  BenchmarkResponse.ok(hashed.to_s)
end
