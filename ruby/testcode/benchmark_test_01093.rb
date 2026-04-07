require_relative 'shared'
require 'uri'

def redirect_uri_join_taint(req)
  dest = URI.join('https://example.com', req.param('path')).to_s
  BenchmarkResponse.new(302, { 'Location' => dest }, '')
end
