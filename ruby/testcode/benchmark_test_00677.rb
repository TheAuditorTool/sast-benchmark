require_relative 'shared'

begin
  require 'bcrypt'
rescue LoadError
end

def store_password_devise(req)
  password = req.param('password')
  hashed = BCrypt::Password.create(password)
  BenchmarkResponse.json({ hash: hashed.to_s })
end
