require_relative 'shared'

def echo_forwarded_host(req)
  host = req.header('HTTP_HOST')
  response = BenchmarkResponse.ok('ok')
  response.headers['X-Forwarded-Host'] = host
  response
end
