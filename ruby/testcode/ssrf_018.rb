require_relative 'shared'
require 'net/http'
require 'uri'

BASE_URL = 'https://api.example.com'

# vuln-code-snippet start ruby_ssrf_base_path
def fetch_api(req)
  path = req.param('path')
  uri = URI.parse("#{BASE_URL}/#{path}") # vuln-code-snippet safe-line ruby_ssrf_base_path
  BenchmarkResponse.ok("fetched: #{uri}")
end
# vuln-code-snippet end ruby_ssrf_base_path
