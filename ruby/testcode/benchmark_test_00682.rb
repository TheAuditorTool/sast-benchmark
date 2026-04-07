require_relative 'shared'

def get_users_with_role(req)
  role_id = req.param('role_id')
  users = User.joins("LEFT JOIN roles ON roles.id = #{role_id}")
  BenchmarkResponse.json({ users: users.map { |u| { id: u[:id] } } })
end
