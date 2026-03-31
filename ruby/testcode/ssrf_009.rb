require_relative 'shared'

module RestClient
  def self.get(url, opts = {})
    "mocked restclient response from #{url}"
  end
end

# vuln-code-snippet start ruby_ssrf_restclient
def fetch_remote(req)
  url = req.param('url')
  response = RestClient.get(url) # vuln-code-snippet vuln-line ruby_ssrf_restclient
  BenchmarkResponse.ok(response.to_s)
end
# vuln-code-snippet end ruby_ssrf_restclient
