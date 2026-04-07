require_relative 'shared'
require 'net/http'

# vuln-code-snippet start ruby_ssrf_net_http_no_redirects
def fetch_hardcoded_host(req)
  http = Net::HTTP.new('hardcoded.example.com', 443)
  http.use_ssl = true
  http.request(Net::HTTP::Get.new('/data')) # vuln-code-snippet safe-line ruby_ssrf_net_http_no_redirects
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssrf_net_http_no_redirects
