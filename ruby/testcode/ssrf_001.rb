require_relative 'shared'
require 'net/http'
require 'uri'

# vuln-code-snippet start ruby_ssrf_net_http
def fetch_url(req)
  url = req.param('url')
  response = Net::HTTP.get(URI(url)) # vuln-code-snippet vuln-line ruby_ssrf_net_http
  BenchmarkResponse.ok(response)
end
# vuln-code-snippet end ruby_ssrf_net_http
