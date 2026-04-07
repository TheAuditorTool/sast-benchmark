require_relative 'shared'

def set_accept_ranges(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Accept-Ranges'] = req.param('unit')
  response
end
