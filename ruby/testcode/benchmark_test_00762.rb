require_relative 'shared'
require 'uri'

def redirect_only_path_helper(req)
  url = req.param('next')
  path = URI.parse(url.start_with?('/') ? url : '/').path
  BenchmarkResponse.new(302, { 'Location' => path }, '')
end
