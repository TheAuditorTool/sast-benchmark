require_relative 'shared'
require 'digest'

def generate_password_reset_token(req)
  username = req.post('username')
  token = Digest::MD5.hexdigest(username + Time.now.to_s)
  BenchmarkResponse.ok("Reset token: #{token}")
end
