require_relative 'shared'

def set_pragma(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Pragma'] = "no-cache, #{req.param('directive')}"
  response
end
