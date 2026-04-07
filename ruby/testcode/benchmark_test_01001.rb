require_relative 'shared'
require 'uri'

def redirect_path_only_uri(req)
  uri = URI.parse(req.param('next'))
  path = uri.path || '/'
  BenchmarkResponse.new(302, { 'Location' => path }, '')
end
