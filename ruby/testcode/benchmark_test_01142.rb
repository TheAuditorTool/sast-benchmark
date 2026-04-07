require_relative 'shared'
require 'uri'

ALLOWED_DOMAINS = %w[api.example.com cdn.example.com].freeze

def fetch_checked(req)
  url = req.param('url')
  parsed = URI.parse(url) rescue nil
  return BenchmarkResponse.bad_request('invalid') unless parsed && ALLOWED_DOMAINS.include?(parsed.host)
  BenchmarkResponse.ok("fetched: #{url}")
end
