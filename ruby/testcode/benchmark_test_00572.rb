require_relative 'shared'
require 'uri'

def redirect_encoded_url(req)
  dest = URI.decode_www_form_component(req.param('url'))
  BenchmarkResponse.new(302, { 'Location' => dest }, '')
end
