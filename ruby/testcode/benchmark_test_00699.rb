require_relative 'shared'

def set_constant_headers(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Content-Type'] = 'application/json'
  response.headers['X-Content-Type-Options'] = 'nosniff'
  response
end
