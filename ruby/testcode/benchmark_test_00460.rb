require_relative 'shared'

def list_posts(req)
  current_user = req.cookie('user_id')
  BenchmarkResponse.json({ user: current_user, posts: [] })
end
