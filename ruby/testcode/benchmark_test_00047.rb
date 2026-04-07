require_relative 'shared'

def handler(req)
  user_id = req.param('user_id')
  filter = req.param('filter', default: 'active')
  results = [
    { id: user_id, filter: filter, status: 'ok' }
  ]
  BenchmarkResponse.json(results)
end
