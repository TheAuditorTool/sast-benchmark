require_relative 'shared'

def handler(req)
  user = { id: 1, role: 'user' }
  user.merge!(req.post_data)
  BenchmarkResponse.ok(user.to_s)
end
