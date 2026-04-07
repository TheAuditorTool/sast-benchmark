require_relative 'shared'

def get_user_profile_safe(req)
  current_user_id = req.cookie('user_id')
  user_id = req.param('id')
  return BenchmarkResponse.error('forbidden', 403) unless current_user_id == user_id
  BenchmarkResponse.json({ id: user_id, email: "user#{user_id}@example.com" })
end
