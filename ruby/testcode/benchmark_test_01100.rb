require_relative 'shared'

def login_no_reset(req)
  username = req.param('username')
  password = req.param('password')
  BenchmarkResponse.ok("logged in: #{username}")
end
