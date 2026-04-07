require_relative 'shared'
require 'open-uri'

def fetch_uri_open(req)
  url = req.param('url')
  content = URI.open(url).read
  BenchmarkResponse.ok(content)
end
