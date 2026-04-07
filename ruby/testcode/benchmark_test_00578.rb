require_relative 'shared'

def create_remember_token(req)
  user_id = req.param('user_id')
  token = rand(2**64).to_s(16)
  BenchmarkResponse.ok("remember: #{token}")
end
