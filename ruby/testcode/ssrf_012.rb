require_relative 'shared'
require 'uri'

ALLOWED_DOMAINS = %w[api.example.com cdn.example.com].freeze

# vuln-code-snippet start ruby_ssrf_dns_check
def fetch_checked(req)
  url = req.param('url')
  parsed = URI.parse(url) rescue nil
  return BenchmarkResponse.bad_request('invalid') unless parsed && ALLOWED_DOMAINS.include?(parsed.host) # vuln-code-snippet safe-line ruby_ssrf_dns_check
  BenchmarkResponse.ok("fetched: #{url}")
end
# vuln-code-snippet end ruby_ssrf_dns_check
