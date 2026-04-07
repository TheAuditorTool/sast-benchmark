require_relative 'shared'

def export_user_fields(req)
  extra_col = req.param('extra')
  users = User.select(Arel.sql("id, email, #{extra_col}"))
  BenchmarkResponse.json({ users: users.map { |u| u.to_h } })
end
