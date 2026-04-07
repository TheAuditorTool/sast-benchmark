require_relative 'shared'
require 'net/http'
require 'uri'

ALLOWED_DOMAINS = %w[api.example.com cdn.example.com feeds.example.com].freeze

def proxy_request(req)
  url = req.param('url')
  uri = URI.parse(url)
  unless ALLOWED_DOMAINS.include?(uri.host)
    return BenchmarkResponse.bad_request("domain not allowed")
  end
  response = Net::HTTP.get(uri)
  BenchmarkResponse.ok(response)
end
