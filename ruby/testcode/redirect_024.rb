require_relative 'shared'
require 'uri'

# vuln-code-snippet start ruby_redirect_uri_join_taint
def redirect_uri_join_taint(req)
  dest = URI.join('https://example.com', req.param('path')).to_s # vuln-code-snippet vuln-line ruby_redirect_uri_join_taint
  BenchmarkResponse.new(302, { 'Location' => dest }, '')
end
# vuln-code-snippet end ruby_redirect_uri_join_taint
