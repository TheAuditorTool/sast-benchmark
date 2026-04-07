require_relative 'shared'

begin
  require 'bcrypt'
rescue LoadError
end

def verify_password_bcrypt(req)
  provided     = req.param('password')
  stored_hash  = req.param('hash')
  match = BCrypt::Password.new(stored_hash).is_password?(provided)
  BenchmarkResponse.json({ match: match })
end
