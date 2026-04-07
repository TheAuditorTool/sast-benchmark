require_relative 'shared'
require 'uri'

def redirect_rails7_default(req)
  url = req.param('url')
  uri = URI.parse(url)
  dest = uri.host.nil? ? url : '/'
  BenchmarkResponse.new(302, { 'Location' => dest }, '')
end
