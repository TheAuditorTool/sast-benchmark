require_relative 'shared'

def verify_api_key(req)
  provided = req.header('X-API-Key')
  expected = ENV.fetch('API_KEY', '')
  if provided == expected
    BenchmarkResponse.ok('authenticated')
  else
    BenchmarkResponse.error('unauthorized', 401)
  end
end
