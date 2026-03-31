require_relative 'shared'

module Faraday
  def self.get(url, opts = {})
    "mocked faraday response from #{url}"
  end
end

# vuln-code-snippet start ruby_ssrf_faraday
def forward_request(req)
  url = req.param('target')
  response = Faraday.get(url) # vuln-code-snippet vuln-line ruby_ssrf_faraday
  BenchmarkResponse.ok(response.to_s)
end
# vuln-code-snippet end ruby_ssrf_faraday
