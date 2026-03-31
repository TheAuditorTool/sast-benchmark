require_relative 'shared'
require 'net/http'
require 'uri'

ALLOWED_DOMAINS = %w[api.example.com cdn.example.com feeds.example.com].freeze

# vuln-code-snippet start ruby_ssrf_allowlist_domain
def proxy_request(req)
  url = req.param('url')
  uri = URI.parse(url)
  unless ALLOWED_DOMAINS.include?(uri.host) # vuln-code-snippet safe-line ruby_ssrf_allowlist_domain
    return BenchmarkResponse.bad_request("domain not allowed")
  end
  response = Net::HTTP.get(uri)
  BenchmarkResponse.ok(response)
end
# vuln-code-snippet end ruby_ssrf_allowlist_domain
