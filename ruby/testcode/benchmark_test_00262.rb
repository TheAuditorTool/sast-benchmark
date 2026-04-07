require_relative 'shared'
require 'digest'

def store_password(req)
  password = req.post('password')
  hashed = Digest::SHA256.hexdigest(password)
  BenchmarkResponse.ok(hashed)
end
