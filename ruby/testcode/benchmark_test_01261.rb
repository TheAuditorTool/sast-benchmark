require_relative 'shared'
require 'digest'

def store_password_sha512(req)
  password = req.param('password')
  digest = Digest::SHA512.hexdigest(password)
  BenchmarkResponse.json({ hash: digest })
end
