require_relative 'shared'

module Excon
  def self.get(url, opts = {}); OpenStruct.new(body: url); end
end

def fetch_excon(req)
  url = req.param('url')
  response = Excon.get(url)
  BenchmarkResponse.ok(response.body)
end
