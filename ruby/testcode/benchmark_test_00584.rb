require_relative 'shared'

def find_user_by_id_arel(req)
  raw_id = req.param('id')
  users = User.where(User.arel_table[:id].eq(raw_id.to_i))
  BenchmarkResponse.json({ user: users.first&.to_h })
end
