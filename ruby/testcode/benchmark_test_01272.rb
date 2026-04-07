require_relative 'shared'
require 'uri'

def redirect_addressable_validate(req)
  u = URI.parse(req.param('url'))
  dest = u.host.nil? ? req.param('url') : '/'
  BenchmarkResponse.new(302, { 'Location' => dest }, '')
end
