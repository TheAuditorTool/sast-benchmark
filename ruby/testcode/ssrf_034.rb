require_relative 'shared'
require 'net/http'
require 'uri'

ALLOWED_HOSTS = %w[api.partner.com cdn.example.com].freeze

# vuln-code-snippet start ruby_ssrf_allowlist_strict
def fetch_allowlisted(req)
  uri = URI.parse(req.param('url'))
  raise 'blocked' unless ALLOWED_HOSTS.include?(uri.host) # vuln-code-snippet safe-line ruby_ssrf_allowlist_strict
  Net::HTTP.get(uri)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssrf_allowlist_strict
