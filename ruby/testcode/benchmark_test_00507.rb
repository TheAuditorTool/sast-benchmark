require_relative 'shared'
require 'digest'

STATIC_SALT = 'deadbeefdeadbeef'

def store_password_static_salt(req)
  password = req.param('password')
  digest = Digest::SHA256.hexdigest(STATIC_SALT + password)
  BenchmarkResponse.json({ hash: digest })
end
