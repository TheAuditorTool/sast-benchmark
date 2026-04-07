require_relative 'shared'
require 'digest'

def store_password_truncated(req)
  password = req.param('password')
  digest = Digest::SHA256.hexdigest(password)[0..15]
  BenchmarkResponse.json({ hash: digest })
end
