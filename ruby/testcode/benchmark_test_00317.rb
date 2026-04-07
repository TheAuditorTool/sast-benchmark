require_relative 'shared'
require 'uri'

def redirect_encoded(req)
  url = req.param('url')
  safe_url = URI.encode_www_form_component(url)
  BenchmarkResponse.new(302, '', { 'Location' => "/redirect?to=#{safe_url}" })
end
