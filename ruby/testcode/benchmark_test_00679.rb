require_relative 'shared'
require 'net/http'
require 'uri'

begin; require 'httpx'; rescue LoadError; end

def fetch_httpx(req)
  url = req.param('url')
  HTTPX.get(url)
  BenchmarkResponse.json({ ok: true })
end
