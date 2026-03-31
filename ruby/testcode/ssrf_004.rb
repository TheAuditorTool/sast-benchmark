require_relative 'shared'
require 'net/http'
require 'uri'

# vuln-code-snippet start ruby_ssrf_scheme_check
def fetch_https_only(req)
  url = req.param('url')
  uri = URI.parse(url)
  unless uri.scheme == 'https' # vuln-code-snippet safe-line ruby_ssrf_scheme_check
    return BenchmarkResponse.bad_request("only https allowed")
  end
  response = Net::HTTP.get(uri)
  BenchmarkResponse.ok(response)
end
# vuln-code-snippet end ruby_ssrf_scheme_check
