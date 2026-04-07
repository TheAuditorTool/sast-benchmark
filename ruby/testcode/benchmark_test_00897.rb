require_relative 'shared'

def set_retry_after(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Retry-After'] = req.param('seconds')
  response
end
