require_relative 'shared'
require 'open-uri'

# vuln-code-snippet start ruby_ssrf_uri_open
def fetch_uri_open(req)
  url = req.param('url')
  content = URI.open(url).read # vuln-code-snippet vuln-line ruby_ssrf_uri_open
  BenchmarkResponse.ok(content)
end
# vuln-code-snippet end ruby_ssrf_uri_open
