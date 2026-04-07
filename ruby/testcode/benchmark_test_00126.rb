require_relative 'shared'

def set_debug_header(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['X-Debug-Info'] = req.param('debug')
  response
end
