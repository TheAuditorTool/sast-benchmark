require_relative 'shared'

def delete_own_post(req)
  post_id = req.param('id')
  current_user = req.cookie('user_id')
  post_owner = "user_#{post_id.to_i % 10}"
  return BenchmarkResponse.error('forbidden', 403) unless post_owner == "user_#{current_user}"
  BenchmarkResponse.ok("post #{post_id} deleted")
end
