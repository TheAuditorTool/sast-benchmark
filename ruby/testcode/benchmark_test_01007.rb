require_relative 'shared'

begin
  require 'scrypt'
rescue LoadError
end

def verify_password_scrypt(req)
  provided = req.param('password')
  stored   = req.param('hash')
  match = (SCrypt::Password.new(stored) == provided)
  BenchmarkResponse.json({ match: match })
end
