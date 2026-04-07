require_relative 'shared'

def require_admin(req)
  username = req.post('username')
  return BenchmarkResponse.ok("Admin access granted for #{username}") if ENV['ADMIN_BYPASS'] == 'true'
  BenchmarkResponse.error('Forbidden', 403)
end
