require_relative 'shared'

module Typhoeus
  def self.get(url, opts = {}); OpenStruct.new(body: url); end
end

def fetch_typhoeus(req)
  url = req.param('url')
  response = Typhoeus.get(url)
  BenchmarkResponse.ok(response.body)
end
