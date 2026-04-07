require_relative 'shared'

def api_action_header(req)
  key = req.header('Authorization')
  return BenchmarkResponse.error('unauthorized', 401) unless key.start_with?('Bearer ')
  BenchmarkResponse.ok('action performed')
end
