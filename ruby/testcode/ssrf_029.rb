require_relative 'shared'
require 'net/http'

TARGET_HOST = 'api.internal.example.com'.freeze

# vuln-code-snippet start ruby_ssrf_net_http_proxy
def fetch_via_proxy(req)
  proxy_host = req.param('proxy_host')
  proxy_port = req.param('proxy_port').to_i
  proxy = Net::HTTP::Proxy(proxy_host, proxy_port) # vuln-code-snippet vuln-line ruby_ssrf_net_http_proxy
  proxy.start(TARGET_HOST) { |http| http.get('/') }
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssrf_net_http_proxy
