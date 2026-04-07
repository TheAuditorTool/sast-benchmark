require_relative 'shared'
require 'net/http'
require 'uri'

# vuln-code-snippet start ruby_ssrf_scheme_only_https
def fetch_https_only(req)
  uri = URI.parse(req.param('url'))
  raise 'only https allowed' unless uri.scheme == 'https' # vuln-code-snippet safe-line ruby_ssrf_scheme_only_https
  Net::HTTP.get(uri)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssrf_scheme_only_https
