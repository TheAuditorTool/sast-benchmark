require_relative 'shared'

def set_refresh_header(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Refresh'] = "0; url=#{req.param('url')}"
  response
end
