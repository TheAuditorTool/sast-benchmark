require_relative 'shared'

def set_cors_origin(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Access-Control-Allow-Origin'] = req.param('origin')
  response
end
