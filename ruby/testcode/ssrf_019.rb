require_relative 'shared'
require 'net/http'
require 'uri'

begin; require 'httpx'; rescue LoadError; end

# vuln-code-snippet start ruby_ssrf_httpx
def fetch_httpx(req)
  url = req.param('url')
  HTTPX.get(url) # vuln-code-snippet vuln-line ruby_ssrf_httpx
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssrf_httpx
