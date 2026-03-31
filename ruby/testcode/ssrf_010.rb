require_relative 'shared'
require 'resolv'
require 'uri'
require 'net/http'

# vuln-code-snippet start ruby_ssrf_private_ip_check
def safe_fetch(req)
  url = req.param('url')
  uri = URI.parse(url)
  ip = Resolv.getaddress(uri.host)
  parts = ip.split('.').map(&:to_i)
  private = (parts[0] == 10) ||
            (parts[0] == 192 && parts[1] == 168) ||
            (parts[0] == 172 && parts[1].between?(16, 31))
  return BenchmarkResponse.bad_request("private addresses not allowed") if private # vuln-code-snippet safe-line ruby_ssrf_private_ip_check
  response = Net::HTTP.get(uri)
  BenchmarkResponse.ok(response)
end
# vuln-code-snippet end ruby_ssrf_private_ip_check
