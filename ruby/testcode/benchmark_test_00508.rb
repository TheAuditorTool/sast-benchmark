require_relative 'shared'
require 'net/http'
require 'uri'

def fetch_url(req)
  url = req.param('url')
  response = Net::HTTP.get(URI(url))
  BenchmarkResponse.ok(response)
end
