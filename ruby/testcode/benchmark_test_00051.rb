require_relative 'shared'

def get_user_profile(req)
  user_id = req.param('id')
  BenchmarkResponse.json({ id: user_id, email: "user#{user_id}@example.com" })
end
