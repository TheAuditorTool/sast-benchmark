require_relative '../../testcode/shared'
require 'net/http'
require 'uri'

SINATRA_ALLOWED_DOMAINS = %w[api.example.com cdn.example.com feeds.example.com].freeze

# vuln-code-snippet start si_ssrf_get
def si_ssrf_get(req)
  url = req.param('url')
  response = Net::HTTP.get(URI(url))  # vuln-code-snippet vuln-line si_ssrf_get
  BenchmarkResponse.ok(response)
end
# vuln-code-snippet end si_ssrf_get

# vuln-code-snippet start si_ssrf_allowlist
def si_ssrf_allowlist(req)
  url = req.param('url')
  uri = URI.parse(url)
  unless SINATRA_ALLOWED_DOMAINS.include?(uri.host)  # vuln-code-snippet safe-line si_ssrf_allowlist
    return BenchmarkResponse.bad_request('domain not in allowlist')
  end
  response = Net::HTTP.get(uri)
  BenchmarkResponse.ok(response)
end
# vuln-code-snippet end si_ssrf_allowlist
