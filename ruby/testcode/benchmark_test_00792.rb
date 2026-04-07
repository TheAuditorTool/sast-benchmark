require_relative 'shared'
require 'openssl'

def store_password_openssl_sha256(req)
  password = req.param('password')
  digest = OpenSSL::Digest.new('SHA256').hexdigest(password)
  BenchmarkResponse.json({ hash: digest })
end
