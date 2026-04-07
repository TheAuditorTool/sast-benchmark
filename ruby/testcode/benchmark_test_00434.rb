require_relative 'shared'
require 'openssl'

def store_password_hmac_sha1(req)
  password = req.param('password')
  secret   = ENV.fetch('APP_SECRET', 'default_secret')
  digest = OpenSSL::HMAC.hexdigest('SHA1', secret, password)
  BenchmarkResponse.json({ hash: digest })
end
