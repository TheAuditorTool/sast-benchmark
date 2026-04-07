require_relative 'shared'
require 'net/http'
require 'uri'

SAFE_API_BASE = 'https://api.trusted.com/v1'.freeze

# vuln-code-snippet start ruby_ssrf_path_only_append
def fetch_path_only(req)
  path = File.basename(req.param('path').gsub('..', ''))
  Net::HTTP.get(URI.parse("#{SAFE_API_BASE}/#{path}")) # vuln-code-snippet safe-line ruby_ssrf_path_only_append
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssrf_path_only_append
