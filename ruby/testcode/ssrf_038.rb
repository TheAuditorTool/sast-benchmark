require_relative 'shared'
require 'net/http'
require 'uri'

BASE_API_URL = 'https://api.trusted.com/v1'.freeze

# vuln-code-snippet start ruby_ssrf_hardcoded_base_url
def fetch_api_path(req)
  path = File.basename(req.param('path'))
  Net::HTTP.get(URI.parse("#{BASE_API_URL}/#{path}")) # vuln-code-snippet safe-line ruby_ssrf_hardcoded_base_url
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssrf_hardcoded_base_url
