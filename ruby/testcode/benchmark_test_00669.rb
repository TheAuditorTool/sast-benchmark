require_relative 'shared'
require 'openssl'

def hash_password_sha1(req)
  password = req.param('password')
  hashed = OpenSSL::Digest.new('SHA1').hexdigest(password)
  BenchmarkResponse.ok(hashed)
end
