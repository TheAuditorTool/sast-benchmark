require_relative 'shared'
require 'uri'

# vuln-code-snippet start ruby_redirect_parsed_same_host
def redirect_parsed_same_host(req)
  url = req.param('url')
  u = URI.parse(url)
  dest = (u.host.nil? || u.host == 'example.com') ? url : '/'
  BenchmarkResponse.new(302, { 'Location' => dest }, '') # vuln-code-snippet safe-line ruby_redirect_parsed_same_host
end
# vuln-code-snippet end ruby_redirect_parsed_same_host
