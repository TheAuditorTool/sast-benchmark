require_relative 'shared'
require 'digest'

def store_password_md5_email(req)
  email    = req.param('email')
  password = req.param('password')
  digest = Digest::MD5.hexdigest(email + ':' + password)
  BenchmarkResponse.json({ hash: digest })
end
