require_relative 'shared'
require 'digest'

def store_password_time_salt(req)
  password = req.param('password')
  digest = Digest::SHA256.hexdigest(password + Time.now.to_s)
  BenchmarkResponse.json({ hash: digest })
end
