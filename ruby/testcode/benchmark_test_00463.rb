require_relative 'shared'

def set_www_authenticate(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['WWW-Authenticate'] = "Bearer realm=\"#{req.param('realm')}\""
  response
end
