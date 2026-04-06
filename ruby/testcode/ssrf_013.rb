require_relative 'shared'

module Excon
  def self.get(url, opts = {}); OpenStruct.new(body: url); end
end

# vuln-code-snippet start ruby_ssrf_excon
def fetch_excon(req)
  url = req.param('url')
  response = Excon.get(url) # vuln-code-snippet vuln-line ruby_ssrf_excon
  BenchmarkResponse.ok(response.body)
end
# vuln-code-snippet end ruby_ssrf_excon
