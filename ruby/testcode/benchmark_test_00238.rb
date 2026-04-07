require_relative 'shared'
require 'digest'

def generate_auth_token(req)
  password = req.post('password')
  token = Digest::SHA1.hexdigest(password)
  BenchmarkResponse.ok(token)
end
