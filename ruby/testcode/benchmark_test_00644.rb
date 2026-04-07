require_relative 'shared'
require 'digest'

def store_password_sha256(req)
  password = req.param('password')
  digest = Digest::SHA256.hexdigest(password)
  BenchmarkResponse.json({ hash: digest })
end
