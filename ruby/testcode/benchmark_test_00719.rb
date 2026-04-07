require_relative 'shared'

begin
  require 'bcrypt'
rescue LoadError
end

def store_password_bcrypt(req)
  password = req.param('password')
  hashed = BCrypt::Password.create(password, cost: 12)
  BenchmarkResponse.json({ hash: hashed.to_s })
end
