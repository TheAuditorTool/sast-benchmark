require_relative 'shared'
require 'digest'

def hash_password_nosalt(req)
  password = req.param('password')
  hashed = Digest::SHA256.hexdigest(password)
  BenchmarkResponse.ok(hashed)
end
