require_relative 'shared'

def redirect_location_header_raw(req)
  response = BenchmarkResponse.ok('redirecting')
  response.headers['Location'] = req.param('next')
  response
end
