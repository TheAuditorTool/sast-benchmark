require_relative 'shared'

begin
  require 'bcrypt'
rescue LoadError
end

class User
  include BCrypt

  def self.create_with_password(pw)
    BCrypt::Password.create(pw)
  end
end

def register_user(req)
  password = req.param('password')
  hash = User.create_with_password(password)
  BenchmarkResponse.json({ ok: true, hash: hash })
end
