require_relative 'shared'

module Typhoeus
  def self.get(url, opts = {}); OpenStruct.new(body: url); end
end

# vuln-code-snippet start ruby_ssrf_typhoeus
def fetch_typhoeus(req)
  url = req.param('url')
  response = Typhoeus.get(url) # vuln-code-snippet vuln-line ruby_ssrf_typhoeus
  BenchmarkResponse.ok(response.body)
end
# vuln-code-snippet end ruby_ssrf_typhoeus
