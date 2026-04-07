require_relative 'shared'

module Argon2
  class Password
    def self.create(password, opts = {}); "argon2:#{password}"; end
  end
end

def hash_password_argon(req)
  password = req.param('password')
  hashed = Argon2::Password.create(password)
  BenchmarkResponse.ok(hashed)
end
