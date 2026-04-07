require_relative 'shared'

module SSRFFilterClient
  def self.get(url); "safe:#{url}"; end
end

# vuln-code-snippet start ruby_ssrf_ssrf_filter_gem
def fetch_filtered(req)
  SSRFFilterClient.get(req.param('url')) # vuln-code-snippet safe-line ruby_ssrf_ssrf_filter_gem
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssrf_ssrf_filter_gem
