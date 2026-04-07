require_relative 'shared'

def set_vary(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Vary'] = "Accept-Encoding, #{req.param('header')}"
  response
end
