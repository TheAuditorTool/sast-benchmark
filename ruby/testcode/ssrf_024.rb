require_relative 'shared'
require 'open-uri'
require 'uri'

# vuln-code-snippet start ruby_ssrf_uri_open_redirect
def fetch_open_uri(req)
  data = URI.open(req.param('url')).read # vuln-code-snippet vuln-line ruby_ssrf_uri_open_redirect
  BenchmarkResponse.ok(data)
end
# vuln-code-snippet end ruby_ssrf_uri_open_redirect
