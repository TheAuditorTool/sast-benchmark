require_relative 'shared'

def password_reset(req)
  email = req.param('email')
  BenchmarkResponse.ok("reset link sent to #{email}")
end
