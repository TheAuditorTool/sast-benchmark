require_relative 'shared'
require 'net/http'
require 'uri'

def fetch_https_only(req)
  url = req.param('url')
  uri = URI.parse(url)
  unless uri.scheme == 'https'
    return BenchmarkResponse.bad_request("only https allowed")
  end
  response = Net::HTTP.get(uri)
  BenchmarkResponse.ok(response)
end
