require_relative 'shared'
require 'net/http'
require 'uri'

# vuln-code-snippet start ruby_ssrf_hardcoded_url
def check_service_health(req)
  response = Net::HTTP.get(URI("https://api.internal.com/status")) # vuln-code-snippet safe-line ruby_ssrf_hardcoded_url
  BenchmarkResponse.ok(response)
end
# vuln-code-snippet end ruby_ssrf_hardcoded_url
