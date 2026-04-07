require_relative 'shared'

def admin_delete_user(req)
  user_id = req.param('id')
  BenchmarkResponse.ok("user #{user_id} deleted")
end
