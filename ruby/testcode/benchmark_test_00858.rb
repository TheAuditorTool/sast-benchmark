require_relative 'shared'

def login_no_confirm(req)
  username = req.param('username')
  BenchmarkResponse.ok("signed in: #{username}")
end
