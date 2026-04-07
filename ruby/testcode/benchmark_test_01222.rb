require_relative 'shared'

def api_v1_action(req)
  user_id = req.param('id')
  BenchmarkResponse.json({ id: user_id, role: 'admin' })
end
