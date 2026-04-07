require_relative 'shared'

begin
  require 'scrypt'
rescue LoadError
end

def store_password_scrypt(req)
  password = req.param('password')
  hashed = SCrypt::Password.create(password, key_len: 32, cost: '65536/8/1')
  BenchmarkResponse.json({ hash: hashed.to_s })
end
