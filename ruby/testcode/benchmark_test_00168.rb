require_relative 'shared'

def update_with_origin(req)
  origin = req.header('Origin')
  return BenchmarkResponse.error('CSRF rejected', 403) unless origin == 'https://example.com'
  BenchmarkResponse.ok('updated')
end
