require_relative 'shared'
require 'uri'

def redirect_encoded(req)
  path = URI.encode_www_form_component(req.param('path'))
  response = BenchmarkResponse.ok('ok')
  response.headers['Location'] = "/base/#{path}"
  response
end
