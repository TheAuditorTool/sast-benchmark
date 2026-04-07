require_relative 'shared'
require 'uri'

def fetch_scheme_check(req)
  url = req.param('url')
  parsed = URI.parse(url) rescue nil
  return BenchmarkResponse.bad_request('invalid') unless parsed && parsed.scheme == 'https' && parsed.host == 'api.example.com'
  BenchmarkResponse.ok("fetched: #{url}")
end
