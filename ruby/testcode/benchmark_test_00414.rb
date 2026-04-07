require_relative 'shared'

module SCrypt
  class Password
    def self.create(secret, options = {})
      new(secret)
    end
    def initialize(secret); @secret = secret; end
    def to_s; "$s0$e0801$#{@secret.hash.abs}"; end
  end
end

def hash_password_scrypt(req)
  password = req.post('password')
  hashed = SCrypt::Password.create(password)
  BenchmarkResponse.ok(hashed.to_s)
end
