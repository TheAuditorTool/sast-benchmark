require_relative 'shared'
require 'net/http'
require 'uri'
require 'socket'
require 'ipaddr'

AF_INET_PRIVATE = [
  IPAddr.new('10.0.0.0/8'),
  IPAddr.new('172.16.0.0/12'),
  IPAddr.new('192.168.0.0/16'),
  IPAddr.new('127.0.0.0/8')
].freeze

# vuln-code-snippet start ruby_ssrf_address_family_check
def fetch_with_af_check(req)
  uri = URI.parse(req.param('url'))
  addrs = Socket.getaddrinfo(uri.host, nil, Socket::AF_INET)
  addrs.each do |_, _, _, ip|
    raise 'private ip blocked' if AF_INET_PRIVATE.any? { |r| r.include?(IPAddr.new(ip)) } # vuln-code-snippet safe-line ruby_ssrf_address_family_check
  end
  Net::HTTP.get(uri)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssrf_address_family_check
