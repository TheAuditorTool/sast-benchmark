require_relative 'shared'
require 'net/http'
require 'uri'

# vuln-code-snippet start ruby_ssrf_webhook_delivery
def deliver_webhook(req)
  url = req.param('webhook_url')
  Net::HTTP.get(URI.parse(url)) # vuln-code-snippet vuln-line ruby_ssrf_webhook_delivery
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssrf_webhook_delivery
