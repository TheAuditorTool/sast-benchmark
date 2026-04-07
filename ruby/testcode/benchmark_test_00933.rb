require_relative 'shared'
require 'net/http'
require 'uri'

BASE_URL = 'https://api.example.com'

def fetch_api(req)
  path = req.param('path')
  uri = URI.parse("#{BASE_URL}/#{path}")
  BenchmarkResponse.ok("fetched: #{uri}")
end
