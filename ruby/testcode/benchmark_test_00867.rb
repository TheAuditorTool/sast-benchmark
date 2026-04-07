require_relative 'shared'
require 'uri'

def redirect_parsed_same_host(req)
  url = req.param('url')
  u = URI.parse(url)
  dest = (u.host.nil? || u.host == 'example.com') ? url : '/'
  BenchmarkResponse.new(302, { 'Location' => dest }, '')
end
