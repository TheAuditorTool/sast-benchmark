require_relative 'shared'

begin; require 'async/http'; rescue LoadError; end

# vuln-code-snippet start ruby_ssrf_async_http
def fetch_async(req)
  url = req.param('url')
  Async::HTTP::Internet.new.get(url) # vuln-code-snippet vuln-line ruby_ssrf_async_http
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssrf_async_http
