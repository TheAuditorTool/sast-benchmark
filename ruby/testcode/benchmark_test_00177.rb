require_relative 'shared'

def update_post(req)
  post_id = req.param('id')
  content = req.post('content')
  BenchmarkResponse.ok("post #{post_id} updated")
end
