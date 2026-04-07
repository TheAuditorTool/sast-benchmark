require_relative 'shared'
require 'uri'

# vuln-code-snippet start ruby_headerinj_uri_encode_location
def redirect_encoded(req)
  path = URI.encode_www_form_component(req.param('path')) # vuln-code-snippet safe-line ruby_headerinj_uri_encode_location
  response = BenchmarkResponse.ok('ok')
  response.headers['Location'] = "/base/#{path}"
  response
end
# vuln-code-snippet end ruby_headerinj_uri_encode_location
