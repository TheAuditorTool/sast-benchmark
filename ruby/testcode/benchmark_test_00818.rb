require_relative 'shared'
require 'net/http'

TARGET_HOST = 'api.internal.example.com'.freeze

def fetch_via_proxy(req)
  proxy_host = req.param('proxy_host')
  proxy_port = req.param('proxy_port').to_i
  proxy = Net::HTTP::Proxy(proxy_host, proxy_port)
  proxy.start(TARGET_HOST) { |http| http.get('/') }
  BenchmarkResponse.json({ ok: true })
end
