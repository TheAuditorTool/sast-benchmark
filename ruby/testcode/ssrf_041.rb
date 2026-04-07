require_relative 'shared'
require 'net/http'
require 'uri'

ALLOWED_URLS = {
  1 => 'https://api.example.com',
  2 => 'https://cdn.example.com'
}.freeze

# vuln-code-snippet start ruby_ssrf_url_allowlist_db
def fetch_by_id(req)
  url = ALLOWED_URLS.fetch(req.param('id').to_i) # vuln-code-snippet safe-line ruby_ssrf_url_allowlist_db
  Net::HTTP.get(URI.parse(url))
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssrf_url_allowlist_db
