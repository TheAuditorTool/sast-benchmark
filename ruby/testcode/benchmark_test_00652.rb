require_relative 'shared'

def list_users_by_role(req)
  role = req.param('role')
  include_inactive = req.param('include_inactive') == 'true'

  DB = get_db_connection unless defined?(DB)
  dataset = DB[:users].where(Sequel.lit("role = '#{role}'"))
  dataset = dataset.where(active: true) unless include_inactive
  users = dataset.map { |u| { id: u[:id], name: u[:name], role: u[:role] } }

  BenchmarkResponse.json({ users: users, count: users.length })
end
