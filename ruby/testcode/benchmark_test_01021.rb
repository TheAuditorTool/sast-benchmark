require_relative 'shared'

begin
  require 'argon2'
rescue LoadError
end

def verify_password_argon2(req)
  provided = req.param('password')
  stored   = req.param('hash')
  match = Argon2::Password.new(stored).matches?(provided)
  BenchmarkResponse.json({ match: match })
end
