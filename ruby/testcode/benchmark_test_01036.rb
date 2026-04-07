require_relative 'shared'

def list_users_by_role_and_status(req)
  role = req.param('role', default: 'member')
  active_param = req.param('active', default: 'true')
  active = active_param == 'true'

  results = FakeActiveRecord::Base.where(role: role, active: active)
  users = results.to_a.map { |u| { id: u[:id], role: u[:role] } }

  BenchmarkResponse.json({ users: users, role: role, active: active })
end
