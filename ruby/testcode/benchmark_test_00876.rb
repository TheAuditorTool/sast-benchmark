require_relative 'shared'
require 'net/http'
require 'uri'

ALLOWED_HOSTS = %w[api.partner.com cdn.example.com].freeze

def fetch_allowlisted(req)
  uri = URI.parse(req.param('url'))
  raise 'blocked' unless ALLOWED_HOSTS.include?(uri.host)
  Net::HTTP.get(uri)
  BenchmarkResponse.json({ ok: true })
end
