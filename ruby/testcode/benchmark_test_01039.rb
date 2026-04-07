require_relative 'shared'
require 'digest'

def hash_password(req)
  password = req.post('password')
  hashed = Digest::MD5.hexdigest(password)
  BenchmarkResponse.ok(hashed)
end
