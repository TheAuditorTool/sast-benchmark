require_relative 'shared'
require 'net/http'
require 'uri'
require 'resolv'
require 'ipaddr'

PRIVATE_RANGES = [
  IPAddr.new('10.0.0.0/8'),
  IPAddr.new('172.16.0.0/12'),
  IPAddr.new('192.168.0.0/16')
].freeze

# vuln-code-snippet start ruby_ssrf_private_ip_block
def fetch_external_only(req)
  uri = URI.parse(req.param('url'))
  ip = IPAddr.new(Resolv.getaddress(uri.host))
  raise 'private range blocked' if PRIVATE_RANGES.any? { |r| r.include?(ip) } # vuln-code-snippet safe-line ruby_ssrf_private_ip_block
  Net::HTTP.get(uri)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssrf_private_ip_block
