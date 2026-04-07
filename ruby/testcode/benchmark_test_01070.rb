require_relative 'shared'
require 'open-uri'

def download_resource(req)
  url = req.param('url')
  content = URI.open(url).read
  BenchmarkResponse.ok(content)
end
