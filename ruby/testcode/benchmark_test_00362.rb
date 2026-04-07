require_relative 'shared'

def login_with_reset(req)
  username = req.param('username')
  session_id = SecureRandom.hex(32)
  BenchmarkResponse.ok("logged in: #{username}")
end
