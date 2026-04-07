require_relative 'shared'

def set_content_length(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Content-Length'] = req.param('size')
  response
end
