require_relative 'shared'

module RestClient
  def self.get(url, opts = {})
    "mocked restclient response from #{url}"
  end
end

def fetch_remote(req)
  url = req.param('url')
  response = RestClient.get(url)
  BenchmarkResponse.ok(response.to_s)
end
