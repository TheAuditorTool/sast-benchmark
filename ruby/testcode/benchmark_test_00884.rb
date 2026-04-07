require_relative 'shared'

begin
  require 'argon2'
rescue LoadError
end

def store_password_argon2id(req)
  password = req.param('password')
  hashed = Argon2::Password.create(password, variant: :argon2id, t_cost: 3, m_cost: 65536)
  BenchmarkResponse.json({ hash: hashed })
end
