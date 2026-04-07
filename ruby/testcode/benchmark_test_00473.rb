require_relative 'shared'
require 'net/http'
require 'uri'
require 'resolv'
require 'ipaddr'

SSRF_PRIVATE = [
  IPAddr.new('10.0.0.0/8'),
  IPAddr.new('172.16.0.0/12'),
  IPAddr.new('192.168.0.0/16'),
  IPAddr.new('127.0.0.0/8')
].freeze

def fetch_after_dns_check(req)
  uri = URI.parse(req.param('url'))
  ips = Resolv.getaddresses(uri.host).map { |a| IPAddr.new(a) }
  raise 'no addresses resolved' if ips.empty?
  raise 'private ip blocked' if ips.any? { |ip| SSRF_PRIVATE.any? { |r| r.include?(ip) } }
  Net::HTTP.get(uri)
  BenchmarkResponse.json({ ok: true })
end
