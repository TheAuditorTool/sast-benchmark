require_relative 'shared'
require 'uri'
require 'ipaddr'

PRIVATE_RANGES = [IPAddr.new('10.0.0.0/8'), IPAddr.new('172.16.0.0/12'), IPAddr.new('192.168.0.0/16'), IPAddr.new('127.0.0.0/8')].freeze

# vuln-code-snippet start ruby_ssrf_private_block
def fetch_no_private(req)
  url = req.param('url')
  parsed = URI.parse(url) rescue nil
  return BenchmarkResponse.bad_request('invalid') unless parsed
  ip = IPAddr.new(parsed.host) rescue nil
  return BenchmarkResponse.bad_request('private') if ip && PRIVATE_RANGES.any? { |r| r.include?(ip) } # vuln-code-snippet safe-line ruby_ssrf_private_block
  BenchmarkResponse.ok("fetched: #{url}")
end
# vuln-code-snippet end ruby_ssrf_private_block
