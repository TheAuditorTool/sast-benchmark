require_relative 'shared'

module HTTParty
  def self.get(url, opts = {})
    "mocked response from #{url}"
  end
end

# vuln-code-snippet start ruby_ssrf_httparty
def fetch_webhook(req)
  url = req.param('url')
  response = HTTParty.get(url) # vuln-code-snippet vuln-line ruby_ssrf_httparty
  BenchmarkResponse.ok(response.to_s)
end
# vuln-code-snippet end ruby_ssrf_httparty
