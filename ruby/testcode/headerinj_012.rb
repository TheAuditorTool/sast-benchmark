require_relative 'shared'
require 'uri'

# vuln-code-snippet start ruby_headerinj_uri_encode
def redirect_encoded(req)
  url = req.param('url')
  safe_url = URI.encode_www_form_component(url) # vuln-code-snippet safe-line ruby_headerinj_uri_encode
  BenchmarkResponse.new(302, '', { 'Location' => "/redirect?to=#{safe_url}" })
end
# vuln-code-snippet end ruby_headerinj_uri_encode
