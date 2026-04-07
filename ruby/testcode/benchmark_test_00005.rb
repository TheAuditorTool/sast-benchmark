require_relative 'shared'
require 'open-uri'
require 'uri'

def fetch_open_uri(req)
  data = URI.open(req.param('url')).read
  BenchmarkResponse.ok(data)
end
