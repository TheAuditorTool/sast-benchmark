require_relative 'shared'
require 'uri'

# vuln-code-snippet start ruby_redirect_encoded_url
def redirect_encoded_url(req)
  dest = URI.decode_www_form_component(req.param('url')) # vuln-code-snippet vuln-line ruby_redirect_encoded_url
  BenchmarkResponse.new(302, { 'Location' => dest }, '')
end
# vuln-code-snippet end ruby_redirect_encoded_url
