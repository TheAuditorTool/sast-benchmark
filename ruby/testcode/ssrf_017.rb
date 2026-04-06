require_relative 'shared'

module Curb
  class Easy
    def self.perform(url); new(url); end
    def initialize(url); @url = url; end
    def body_str; @url; end
  end
end

# vuln-code-snippet start ruby_ssrf_curb
def fetch_curb(req)
  url = req.param('url')
  response = Curb::Easy.perform(url) # vuln-code-snippet vuln-line ruby_ssrf_curb
  BenchmarkResponse.ok(response.body_str)
end
# vuln-code-snippet end ruby_ssrf_curb
