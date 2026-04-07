require_relative 'shared'

def api_action_query(req)
  key = req.param('api_key')
  return BenchmarkResponse.error('unauthorized', 401) if key.empty?
  BenchmarkResponse.ok('action performed')
end
