require_relative 'shared'
require 'openssl'

def store_password_rmd160(req)
  password = req.param('password')
  digest = OpenSSL::Digest.new('RIPEMD160').hexdigest(password)
  BenchmarkResponse.json({ hash: digest })
end
