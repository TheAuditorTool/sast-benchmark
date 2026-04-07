require_relative 'shared'

def admin_delete_safe(req)
  role = req.cookie('role')
  return BenchmarkResponse.error('forbidden', 403) unless role == 'admin'
  user_id = req.param('id')
  BenchmarkResponse.ok("user #{user_id} deleted")
end
