require_relative 'shared'

def export_user_profile(req)
  user_id = req.param('user_id').to_i
  user = User.select(:id, :email, :name).find(user_id)
  BenchmarkResponse.json({ user: user&.to_h })
end
