require_relative 'shared'

def authenticate_api_token(req)
  provided = req.header('X-Api-Token')
  stored = ENV.fetch('API_TOKEN', 'secret-token')
  return BenchmarkResponse.error('Unauthorized', 401) unless provided == stored
  BenchmarkResponse.ok('authenticated')
end
