require_relative 'shared'

module Faraday
  def self.get(url, opts = {})
    "mocked faraday response from #{url}"
  end
end

def forward_request(req)
  url = req.param('target')
  response = Faraday.get(url)
  BenchmarkResponse.ok(response.to_s)
end
