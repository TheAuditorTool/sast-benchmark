require_relative 'shared'
require 'open-uri'

# vuln-code-snippet start ruby_ssrf_open_uri
def download_resource(req)
  url = req.param('url')
  content = URI.open(url).read # vuln-code-snippet vuln-line ruby_ssrf_open_uri
  BenchmarkResponse.ok(content)
end
# vuln-code-snippet end ruby_ssrf_open_uri
