require_relative 'shared'
require 'net/http'

# vuln-code-snippet start ruby_ssrf_timeout_no_redirect
def fetch_with_timeouts(req)
  http = Net::HTTP.new('api.example.com', 443)
  http.open_timeout = 5
  http.read_timeout = 5
  http.max_retries = 0
  http.use_ssl = true
  http.get('/data') # vuln-code-snippet safe-line ruby_ssrf_timeout_no_redirect
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssrf_timeout_no_redirect
