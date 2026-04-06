require_relative 'shared'
require 'uri'

# vuln-code-snippet start ruby_ssrf_scheme_host
def fetch_scheme_check(req)
  url = req.param('url')
  parsed = URI.parse(url) rescue nil
  return BenchmarkResponse.bad_request('invalid') unless parsed && parsed.scheme == 'https' && parsed.host == 'api.example.com' # vuln-code-snippet safe-line ruby_ssrf_scheme_host
  BenchmarkResponse.ok("fetched: #{url}")
end
# vuln-code-snippet end ruby_ssrf_scheme_host
