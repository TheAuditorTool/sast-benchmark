require_relative 'shared'

module HTTParty
  def self.get(url, opts = {})
    "mocked response from #{url}"
  end
end

def fetch_webhook(req)
  url = req.param('url')
  response = HTTParty.get(url)
  BenchmarkResponse.ok(response.to_s)
end
