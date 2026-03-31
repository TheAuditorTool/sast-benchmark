require_relative 'shared'
require 'resolv'
require 'uri'
require 'net/http'

INTERNAL_RANGES = [
  /\A10\.\d+\.\d+\.\d+\z/,
  /\A192\.168\.\d+\.\d+\z/,
  /\A172\.(1[6-9]|2\d|3[01])\.\d+\.\d+\z/
].freeze

# vuln-code-snippet start ruby_ssrf_internal_only
def fetch_internal_resource(req)
  url = req.param('url')
  uri = URI.parse(url)
  ip = Resolv.getaddress(uri.host)
  unless INTERNAL_RANGES.any? { |r| ip.match?(r) } # vuln-code-snippet safe-line ruby_ssrf_internal_only
    return BenchmarkResponse.bad_request("external hosts not allowed")
  end
  response = Net::HTTP.get(uri)
  BenchmarkResponse.ok(response)
end
# vuln-code-snippet end ruby_ssrf_internal_only
