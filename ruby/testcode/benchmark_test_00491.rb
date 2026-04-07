require_relative 'shared'

module SCrypt
  class Password
    def self.create(password, opts = {}); "scrypt:#{password}"; end
  end
end

def hash_password_scrypt(req)
  password = req.param('password')
  hashed = SCrypt::Password.create(password)
  BenchmarkResponse.ok(hashed)
end
